//! `i18n-format` — the shared, dependency-free foundation for the i18n system.
//!
//! It contains three things that BOTH the proc-macro crate (at *compile time*)
//! and the runtime crate (at *run time*) need to agree on:
//!
//! 1. The language model: [`Languages`], [`LanguagesWithValue`], [`Translate`].
//! 2. The on-the-wire binary *catalog* layout ([`encode_catalog`] / [`Catalog`]).
//! 3. Rendering: plural selection + argument splicing, with a zero-allocation
//!    fast path for templates that have no placeholders.
//!
//! No `HashMap` anywhere: lookups are a binary search over a packed, sorted key
//! table inside one contiguous byte buffer, and argument lookup is **positional**
//! — an arg is addressed by a `u8` index, never by a name string. The whole
//! catalog is a single `&'static [u8]` (or one `Vec<u8>`), so there is exactly
//! one allocation (or zero, when embedded in `.rodata`) and no per-string heap
//! fragmentation.
//!
//! ## The positional-argument trick (no placeholder names on the wire)
//!
//! A template like `"Page {current} of {total}"` is stored as the *literal*
//! `"Page  of "` plus a tiny table saying "insert arg #0 at offset 5, arg #1 at
//! offset 9". The `u8` arg index is the **rank of the argument name in the
//! sorted set of names for that entry** (the plural driver name is included in
//! the set). The `t!` call site computes the *same* ranks by sorting the names
//! it was given, so both sides agree without the names ever being serialized.
//! That keeps the fetched/embedded catalog tiny and removes all name-matching at
//! render time.

#![forbid(unsafe_code)]

use core::fmt;
use std::borrow::Cow;
use std::collections::BTreeSet;

// ===========================================================================
// 1. Language model
// ===========================================================================

/// Every language the system knows about. The discriminant is stable and is
/// used as an array index (`as usize`) and serialized into catalogs (`as u8`),
/// so never reorder existing variants — only append.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Languages {
    En = 0,
    EnUs = 1,
    Pt = 2,
    PtBr = 3,
}

impl Languages {
    /// Number of known languages — the width of the runtime catalog table.
    pub const COUNT: usize = 4;

    /// All variants, in discriminant order.
    pub const ALL: [Languages; Self::COUNT] = [
        Languages::En,
        Languages::EnUs,
        Languages::Pt,
        Languages::PtBr,
    ];

    /// Parse a BCP-47-ish tag into a language. Accepts `_` or `-` separators and
    /// is case-insensitive: `"en"`, `"EN"`, `"en-US"`, `"en_us"`, `"pt"`,
    /// `"pt-BR"`. This is the "function receiving a `&str` that returns some
    /// `Languages::ENUM_ELEMENT`" requested in the design.
    ///
    /// Region-only tags fall back to the base language (`"en-GB"` -> `En`).
    pub fn parse(tag: &str) -> Option<Languages> {
        let norm: String = tag
            .trim()
            .chars()
            .map(|c| {
                if c == '_' {
                    '-'
                } else {
                    c.to_ascii_lowercase()
                }
            })
            .collect();
        match norm.as_str() {
            "en-us" => Some(Languages::EnUs),
            "pt-br" => Some(Languages::PtBr),
            _ => match norm.split('-').next().unwrap_or("") {
                "en" => Some(Languages::En),
                "pt" => Some(Languages::Pt),
                _ => None,
            },
        }
    }

    /// Inverse of the discriminant. Returns `None` for out-of-range bytes coming
    /// off the wire / from an atomic.
    pub const fn from_index(i: u8) -> Option<Languages> {
        match i {
            0 => Some(Languages::En),
            1 => Some(Languages::EnUs),
            2 => Some(Languages::Pt),
            3 => Some(Languages::PtBr),
            _ => None,
        }
    }

    /// Index for array-based registries (no map needed).
    pub const fn as_index(self) -> usize {
        self as usize
    }

    /// Canonical BCP-47 tag (handy for HTTP `Accept-Language` when fetching from
    /// a server on wasm).
    pub const fn bcp47(self) -> &'static str {
        match self {
            Languages::En => "en",
            Languages::EnUs => "en-US",
            Languages::Pt => "pt",
            Languages::PtBr => "pt-BR",
        }
    }

    /// Fallback chain: a region dialect falls back to its base language; a base
    /// language is its own fallback (terminal).
    pub const fn fallback(self) -> Languages {
        match self {
            Languages::EnUs => Languages::En,
            Languages::PtBr => Languages::Pt,
            base => base,
        }
    }
}

/// A language *paired with* a string value. This is exactly what appears in the
/// `#[traductions]` DSL arrays (`En("January")`, `PtBr("Janeiro")`). The macro
/// consumes these and strips them out of the generated enum, but the type is
/// public so the DSL elements have a real meaning and can be used by hand.
#[derive(Clone, Copy, Debug)]
pub enum LanguagesWithValue {
    En(&'static str),
    EnUs(&'static str),
    Pt(&'static str),
    PtBr(&'static str),
}

impl LanguagesWithValue {
    pub const fn language(self) -> Languages {
        match self {
            LanguagesWithValue::En(_) => Languages::En,
            LanguagesWithValue::EnUs(_) => Languages::EnUs,
            LanguagesWithValue::Pt(_) => Languages::Pt,
            LanguagesWithValue::PtBr(_) => Languages::PtBr,
        }
    }
    pub const fn value(self) -> &'static str {
        match self {
            LanguagesWithValue::En(v)
            | LanguagesWithValue::EnUs(v)
            | LanguagesWithValue::Pt(v)
            | LanguagesWithValue::PtBr(v) => v,
        }
    }
}

/// Implemented by every application enum produced by `#[traductions]`.
///
/// `APP_ID` is a compile-time `u16` (FNV-1a of the enum name + a salt, computed
/// inside the proc-macro). `variant_index` is just `*self as u8`. Together they
/// form the 24-bit composite key (`U24`) used to look a string up.
pub trait Translate: Copy {
    /// Hash of the enum's name, generated at compile time by the macro.
    const APP_ID: u16;

    /// `*self as u8`.
    fn variant_index(self) -> u8;

    /// The `translation_id` requested in the spec — just the application id.
    #[inline]
    fn translation_id(self) -> u16 {
        Self::APP_ID
    }

    /// The packed (u16, u8) -> u32 lookup key.
    #[inline]
    fn key(self) -> u32 {
        composite_key(Self::APP_ID, self.variant_index())
    }
}

/// Pack `(app_id: u16, variant: u8)` into the 24-bit composite key, stored in a
/// `u32`. Ordering this `u32` orders catalog records, which lets us binary
/// search. `const` so it can be used in `const` contexts.
#[inline]
pub const fn composite_key(app_id: u16, variant: u8) -> u32 {
    ((app_id as u32) << 8) | (variant as u32)
}

// ===========================================================================
// 2. Binary catalog format
// ===========================================================================
//
// One contiguous buffer, all little-endian:
//
//   Header (16 bytes)
//     [0..4]   magic  = b"I18N"
//     [4]      version
//     [5]      language discriminant this catalog is for
//     [6..8]   key_count : u16
//     [8..12]  records_offset : u32   (always 16, explicit for forward-compat)
//     [12..16] strings_offset : u32
//
//   Key table: `key_count` fixed 12-byte records, SORTED by composite key:
//     [0..2]   app_id  : u16
//     [2]      variant : u8
//     [3]      flags   : u8     (F_ARGS | F_PLURAL)
//     [4..8]   offset  : u32    (into the strings region)
//     [8..12]  len     : u32    (byte length of the entry blob)
//
//   Strings region: the concatenated entry blobs.
//
//     A "payload" is one rendered form:
//        [quantity:u8]                          number of arg insertions
//        [ (pos:u16, arg_index:u8) * quantity ] where + which arg
//        [size:u32]                             literal byte length
//        [literal: size]                        template with placeholders removed
//
//     plain entry  : one payload.
//     plural entry : [form_count:u8][plural_arg_index:u8] then `form_count`
//                    times: [category:u8] followed by one payload.
//
// `pos` is a byte offset into `literal`; at render time the arg's value is
// spliced in there. `arg_index` is the rank of the arg name in the entry's
// sorted name set (see the module docs). `plural_arg_index` is the rank of the
// declared plural-driver name; the runtime reads that arg's numeric value to
// pick a form. `F_ARGS` is set when any payload has quantity > 0; if it is clear
// the literal is handed back as `Cow::Borrowed(&'static str)` with no alloc.

pub const MAGIC: [u8; 4] = *b"I18N";
pub const VERSION: u8 = 1;
pub const HEADER_LEN: usize = 16;
pub const RECORD_LEN: usize = 12;

pub const F_ARGS: u8 = 0b0000_0001;
pub const F_PLURAL: u8 = 0b0000_0010;

/// CLDR-style plural categories. We keep the full set so the format does not
/// need to change to support Slavic/Arabic rules later; the simplified
/// [`plural_category`] only emits `One`/`Other` for En/Pt today.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Plural {
    Other = 0,
    Zero = 1,
    One = 2,
    Two = 3,
    Few = 4,
    Many = 5,
}

impl Plural {
    pub const fn from_u8(b: u8) -> Plural {
        match b {
            1 => Plural::Zero,
            2 => Plural::One,
            3 => Plural::Two,
            4 => Plural::Few,
            5 => Plural::Many,
            _ => Plural::Other,
        }
    }
}

/// Pick the plural category for a count in a given language. Simplified CLDR:
/// English and Portuguese both use `one` for n == 1 and `other` otherwise.
/// (Portuguese also accepts `0` and `1` as `one` in some data, but the safe
/// subset keeps `one == 1`.) Extend per-language here when you add e.g. Russian.
pub fn plural_category(_lang: Languages, n: i64) -> Plural {
    if n == 1 {
        Plural::One
    } else {
        Plural::Other
    }
}

// ---- Encoder (used by the proc-macro at compile time) ---------------------

/// One translation form (a plural category + its template, placeholders intact).
#[derive(Clone, Debug)]
pub struct FormSpec {
    pub category: Plural,
    pub template: String,
}

/// One translatable entry, prior to encoding.
#[derive(Clone, Debug)]
pub struct EntrySpec {
    pub app_id: u16,
    pub variant: u8,
    /// One form for a plain string, several for a plural string.
    pub forms: Vec<FormSpec>,
    /// `Some(name)` for a plural entry: the argument whose numeric value selects
    /// the form. Its rank in the entry's sorted name set is stored as
    /// `plural_arg_index`. `None` for a plain entry.
    pub plural_driver: Option<String>,
}

/// Walk a template and add every `{name}` placeholder to `names`. `{{` / `}}`
/// are escaped literal braces and are skipped.
fn collect_names(tmpl: &str, names: &mut BTreeSet<String>) {
    let b = tmpl.as_bytes();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'{' if i + 1 < b.len() && b[i + 1] == b'{' => i += 2,
            b'}' if i + 1 < b.len() && b[i + 1] == b'}' => i += 2,
            b'{' => {
                let mut j = i + 1;
                while j < b.len() && b[j] != b'}' {
                    j += 1;
                }
                names.insert(tmpl[i + 1..j].to_string());
                i = if j < b.len() { j + 1 } else { j };
            }
            _ => i += utf8_len(b[i]),
        }
    }
}

/// Split a template into its literal bytes (placeholders removed) and the slot
/// table `(byte_offset_in_literal, arg_index)`. `index_of` maps a placeholder
/// name to its `u8` rank. `{{` / `}}` become single literal braces.
fn split_template(tmpl: &str, index_of: &impl Fn(&str) -> u8) -> (Vec<u8>, Vec<(u16, u8)>) {
    let b = tmpl.as_bytes();
    let mut lit: Vec<u8> = Vec::with_capacity(b.len());
    let mut slots: Vec<(u16, u8)> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'{' if i + 1 < b.len() && b[i + 1] == b'{' => {
                lit.push(b'{');
                i += 2;
            }
            b'}' if i + 1 < b.len() && b[i + 1] == b'}' => {
                lit.push(b'}');
                i += 2;
            }
            b'{' => {
                let mut j = i + 1;
                while j < b.len() && b[j] != b'}' {
                    j += 1;
                }
                let name = &tmpl[i + 1..j];
                slots.push((lit.len() as u16, index_of(name)));
                i = if j < b.len() { j + 1 } else { j };
            }
            _ => {
                let l = utf8_len(b[i]);
                lit.extend_from_slice(&b[i..i + l]);
                i += l;
            }
        }
    }
    (lit, slots)
}

/// Append one payload `[quantity][slots][size][literal]` to `out`. Returns
/// whether it had any args.
fn push_payload(out: &mut Vec<u8>, lit: &[u8], slots: &[(u16, u8)]) -> bool {
    out.push(slots.len() as u8);
    for (pos, idx) in slots {
        out.extend_from_slice(&pos.to_le_bytes());
        out.push(*idx);
    }
    out.extend_from_slice(&(lit.len() as u32).to_le_bytes());
    out.extend_from_slice(lit);
    !slots.is_empty()
}

/// Encode a fully-resolved set of entries (all for one language) into the single
/// contiguous catalog buffer. Entries are sorted by composite key so the runtime
/// can binary-search. This is the "single continuous `Vec<u8>`" the design asks
/// for; the macro embeds the result as a `&'static [u8]` const.
pub fn encode_catalog(lang: Languages, mut entries: Vec<EntrySpec>) -> Vec<u8> {
    entries.sort_by_key(|e| composite_key(e.app_id, e.variant));
    entries.dedup_by_key(|e| composite_key(e.app_id, e.variant));

    let key_count = entries.len();
    let records_offset = HEADER_LEN;
    let strings_offset = HEADER_LEN + key_count * RECORD_LEN;

    // Build the strings region and remember each entry's (offset, len, flags).
    let mut strings: Vec<u8> = Vec::new();
    let mut meta: Vec<(u16, u8, u8, u32, u32)> = Vec::with_capacity(key_count);

    for e in &entries {
        // The per-entry sorted name set: every placeholder across every form,
        // plus the plural driver. Rank in this set is the wire arg index.
        let mut names: BTreeSet<String> = BTreeSet::new();
        for f in &e.forms {
            collect_names(&f.template, &mut names);
        }
        if let Some(d) = &e.plural_driver {
            names.insert(d.clone());
        }
        let index_of = |name: &str| names.iter().position(|n| n == name).unwrap_or(0) as u8;

        let is_plural = e.plural_driver.is_some() && e.forms.len() > 1;
        let mut flags = 0u8;
        let mut has_args = false;

        let start = strings.len() as u32;
        if !is_plural {
            let (lit, slots) = split_template(&e.forms[0].template, &index_of);
            has_args |= push_payload(&mut strings, &lit, &slots);
        } else {
            flags |= F_PLURAL;
            strings.push(e.forms.len() as u8);
            let driver = e.plural_driver.as_deref().unwrap_or("");
            strings.push(index_of(driver));
            for f in &e.forms {
                strings.push(f.category as u8);
                let (lit, slots) = split_template(&f.template, &index_of);
                has_args |= push_payload(&mut strings, &lit, &slots);
            }
        }
        if has_args {
            flags |= F_ARGS;
        }
        let len = strings.len() as u32 - start;
        meta.push((e.app_id, e.variant, flags, start, len));
    }

    let mut buf = Vec::with_capacity(strings_offset + strings.len());
    // Header
    buf.extend_from_slice(&MAGIC);
    buf.push(VERSION);
    buf.push(lang as u8);
    buf.extend_from_slice(&(key_count as u16).to_le_bytes());
    buf.extend_from_slice(&(records_offset as u32).to_le_bytes());
    buf.extend_from_slice(&(strings_offset as u32).to_le_bytes());
    debug_assert_eq!(buf.len(), HEADER_LEN);

    // Key table (already sorted)
    for (app_id, variant, flags, off, len) in &meta {
        buf.extend_from_slice(&app_id.to_le_bytes());
        buf.push(*variant);
        buf.push(*flags);
        buf.extend_from_slice(&off.to_le_bytes());
        buf.extend_from_slice(&len.to_le_bytes());
    }
    debug_assert_eq!(buf.len(), strings_offset);

    // Strings region
    buf.extend_from_slice(&strings);
    buf
}

// ---- Decoder + lookup (used by the runtime) -------------------------------

/// A read-only view over a catalog buffer. `Copy`, holds only a slice — for the
/// embedded case `data` is `&'static [u8]` living in `.rodata`.
#[derive(Clone, Copy)]
pub struct Catalog<'a> {
    data: &'a [u8],
}

/// A located entry: its flags plus the slice of bytes describing its forms.
#[derive(Clone, Copy)]
pub struct Entry<'a> {
    pub flags: u8,
    bytes: &'a [u8],
}

fn rd_u16(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}
fn rd_u32(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

impl<'a> Catalog<'a> {
    /// Wrap a buffer, validating the header. Returns `None` if it is not a
    /// recognizable catalog (wrong magic / version / truncated).
    pub fn new(data: &'a [u8]) -> Option<Catalog<'a>> {
        if data.len() < HEADER_LEN || data[0..4] != MAGIC || data[4] != VERSION {
            return None;
        }
        Some(Catalog { data })
    }

    /// Construct without validation. Intended for the macro-emitted const where
    /// the bytes are known-good; avoids any runtime check.
    pub const fn new_unchecked(data: &'a [u8]) -> Catalog<'a> {
        Catalog { data }
    }

    /// The raw catalog bytes. Used by the generator to re-export a per-language
    /// file that the wasm runtime fetches and installs verbatim.
    pub fn raw_bytes(&self) -> &'a [u8] {
        self.data
    }

    pub fn language(&self) -> Option<Languages> {
        Languages::from_index(self.data[5])
    }

    pub fn key_count(&self) -> usize {
        rd_u16(self.data, 6) as usize
    }

    fn strings_offset(&self) -> usize {
        rd_u32(self.data, 12) as usize
    }

    /// (composite_key, flags, offset, len) for record `i`.
    fn record(&self, i: usize) -> (u32, u8, usize, usize) {
        let base = HEADER_LEN + i * RECORD_LEN;
        let app_id = rd_u16(self.data, base);
        let variant = self.data[base + 2];
        let flags = self.data[base + 3];
        let off = rd_u32(self.data, base + 4) as usize;
        let len = rd_u32(self.data, base + 8) as usize;
        (composite_key(app_id, variant), flags, off, len)
    }

    /// Binary search the sorted key table for `(app_id, variant)`.
    pub fn lookup(&self, app_id: u16, variant: u8) -> Option<Entry<'a>> {
        let want = composite_key(app_id, variant);
        let (mut lo, mut hi) = (0usize, self.key_count());
        while lo < hi {
            let mid = (lo + hi) / 2;
            let (k, flags, off, len) = self.record(mid);
            if k == want {
                let s = self.strings_offset();
                let start = s + off;
                return Some(Entry {
                    flags,
                    bytes: &self.data[start..start + len],
                });
            } else if k < want {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        None
    }
}

/// Read one payload `[quantity][slots][size][literal]` starting at `at`.
/// Returns the slot table, the literal `&str`, and the offset just past it.
fn read_payload(bytes: &[u8], at: usize) -> (Vec<(u16, u8)>, &str, usize) {
    let quantity = bytes[at] as usize;
    let mut cur = at + 1;
    let mut slots = Vec::with_capacity(quantity);
    for _ in 0..quantity {
        slots.push((rd_u16(bytes, cur), bytes[cur + 2]));
        cur += 3;
    }
    let size = rd_u32(bytes, cur) as usize;
    cur += 4;
    let lit = core::str::from_utf8(&bytes[cur..cur + size]).unwrap_or("");
    (slots, lit, cur + size)
}

impl<'a> Entry<'a> {
    /// Render this entry. Zero-allocation `Cow::Borrowed` when the chosen form
    /// has no argument slots; otherwise one `String` is produced.
    pub fn render(&self, lang: Languages, args: &[ArgValue<'_>]) -> Cow<'a, str> {
        if self.flags & F_PLURAL == 0 {
            let (slots, lit, _) = read_payload(self.bytes, 0);
            return render_form(lit, &slots, args);
        }

        // Plural: [form_count][plural_arg_index] then per form [category][payload].
        let form_count = self.bytes[0] as usize;
        let plural_idx = self.bytes[1] as usize;
        let count = args.get(plural_idx).map(ArgValue::as_i64).unwrap_or(0);
        let want = plural_category(lang, count);

        let mut cur = 2usize;
        let mut chosen: Option<(Vec<(u16, u8)>, &str)> = None;
        let mut other: Option<(Vec<(u16, u8)>, &str)> = None;
        for _ in 0..form_count {
            let cat = Plural::from_u8(self.bytes[cur]);
            let (slots, lit, next) = read_payload(self.bytes, cur + 1);
            cur = next;
            if cat == want {
                chosen = Some((slots, lit));
                break;
            }
            if cat == Plural::Other {
                other = Some((slots, lit));
            }
        }
        match chosen.or(other) {
            Some((slots, lit)) => render_form(lit, &slots, args),
            None => Cow::Borrowed(""),
        }
    }
}

/// Splice argument values into a literal at the recorded offsets. Borrowed (zero
/// alloc) when there are no slots.
fn render_form<'a>(lit: &'a str, slots: &[(u16, u8)], args: &[ArgValue<'_>]) -> Cow<'a, str> {
    use std::fmt::Write;
    if slots.is_empty() {
        return Cow::Borrowed(lit);
    }
    let mut out = String::with_capacity(lit.len() + 16);
    let mut cursor = 0usize;
    for (pos, idx) in slots {
        let pos = (*pos as usize).min(lit.len());
        out.push_str(&lit[cursor..pos]);
        match args.get(*idx as usize) {
            Some(v) => {
                let _ = write!(out, "{v}");
            }
            // Missing arg: keep it visible to aid debugging.
            None => {
                let _ = write!(out, "\u{27E6}#{idx}\u{27E7}");
            }
        }
        cursor = pos;
    }
    out.push_str(&lit[cursor..]);
    Cow::Owned(out)
}

// ===========================================================================
// 3. Argument values (positional; addressed by u8 index, never by name)
// ===========================================================================

/// A formatting argument value. The `t!` macro builds a `&[ArgValue]` literal in
/// sorted-name order; the catalog addresses each by its index. Values are
/// `Display`-formatted when spliced into a literal.
#[derive(Clone)]
pub enum ArgValue<'a> {
    Str(&'a str),
    Owned(String),
    I64(i64),
    U64(u64),
    F64(f64),
}

impl ArgValue<'_> {
    /// Numeric view used for plural selection (the driver arg). Non-numeric
    /// strings parse to 0.
    pub fn as_i64(&self) -> i64 {
        match self {
            ArgValue::I64(n) => *n,
            ArgValue::U64(n) => *n as i64,
            ArgValue::F64(n) => *n as i64,
            ArgValue::Str(s) => s.parse().unwrap_or(0),
            ArgValue::Owned(s) => s.parse().unwrap_or(0),
        }
    }
}

impl fmt::Display for ArgValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgValue::Str(s) => f.write_str(s),
            ArgValue::Owned(s) => f.write_str(s),
            ArgValue::I64(n) => write!(f, "{n}"),
            ArgValue::U64(n) => write!(f, "{n}"),
            ArgValue::F64(n) => write!(f, "{n}"),
        }
    }
}

macro_rules! impl_into_argvalue {
    ($t:ty, $variant:ident) => {
        impl<'a> From<$t> for ArgValue<'a> {
            fn from(v: $t) -> Self {
                ArgValue::$variant(v as _)
            }
        }
    };
}
impl_into_argvalue!(i64, I64);
impl_into_argvalue!(i32, I64);
impl_into_argvalue!(u32, U64);
impl_into_argvalue!(u64, U64);
impl_into_argvalue!(usize, U64);
impl_into_argvalue!(f64, F64);
impl_into_argvalue!(f32, F64);

impl<'a> From<&'a str> for ArgValue<'a> {
    fn from(v: &'a str) -> Self {
        ArgValue::Str(v)
    }
}
impl<'a> From<&'a String> for ArgValue<'a> {
    fn from(v: &'a String) -> Self {
        ArgValue::Str(v.as_str())
    }
}
impl<'a> From<String> for ArgValue<'a> {
    fn from(v: String) -> Self {
        ArgValue::Owned(v)
    }
}
impl<'a> From<Cow<'a, str>> for ArgValue<'a> {
    fn from(v: Cow<'a, str>) -> Self {
        match v {
            Cow::Borrowed(s) => ArgValue::Str(s),
            Cow::Owned(s) => ArgValue::Owned(s),
        }
    }
}

#[inline]
fn utf8_len(first: u8) -> usize {
    match first {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        _ => 4,
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(app: u16, var: u8, t: &str) -> EntrySpec {
        EntrySpec {
            app_id: app,
            variant: var,
            forms: vec![FormSpec {
                category: Plural::Other,
                template: t.to_string(),
            }],
            plural_driver: None,
        }
    }

    #[test]
    fn lang_parse() {
        assert_eq!(Languages::parse("en"), Some(Languages::En));
        assert_eq!(Languages::parse("EN"), Some(Languages::En));
        assert_eq!(Languages::parse("en-US"), Some(Languages::EnUs));
        assert_eq!(Languages::parse("en_us"), Some(Languages::EnUs));
        assert_eq!(Languages::parse("pt-BR"), Some(Languages::PtBr));
        assert_eq!(Languages::parse("en-GB"), Some(Languages::En)); // region falls back
        assert_eq!(Languages::parse("xx"), None);
    }

    #[test]
    fn composite_and_fallback() {
        assert_eq!(composite_key(0xABCD, 0x12), 0x00AB_CD12);
        assert_eq!(Languages::PtBr.fallback(), Languages::Pt);
        assert_eq!(Languages::En.fallback(), Languages::En);
    }

    #[test]
    fn roundtrip_plain_zero_alloc() {
        let cat_bytes = encode_catalog(
            Languages::PtBr,
            vec![plain(7, 0, "Janeiro"), plain(7, 1, "Fevereiro")],
        );
        let cat = Catalog::new(&cat_bytes).unwrap();
        assert_eq!(cat.language(), Some(Languages::PtBr));
        assert_eq!(cat.key_count(), 2);

        let jan = cat.lookup(7, 0).unwrap();
        let r = jan.render(Languages::PtBr, &[]);
        assert_eq!(r, "Janeiro");
        assert!(
            matches!(r, Cow::Borrowed(_)),
            "plain string must not allocate"
        );

        assert!(cat.lookup(7, 9).is_none());
        assert!(cat.lookup(8, 0).is_none());
    }

    #[test]
    fn binary_search_many_keys() {
        // Insert out of order; encoder must sort so search works.
        let mut specs = Vec::new();
        for v in (0u8..50).rev() {
            specs.push(plain(3, v, &format!("v{v}")));
        }
        let bytes = encode_catalog(Languages::En, specs);
        let cat = Catalog::new(&bytes).unwrap();
        for v in 0u8..50 {
            let e = cat.lookup(3, v).expect("present");
            assert_eq!(e.render(Languages::En, &[]), format!("v{v}"));
        }
    }

    #[test]
    fn single_placeholder() {
        let bytes = encode_catalog(Languages::En, vec![plain(1, 0, "The month {month}!")]);
        let cat = Catalog::new(&bytes).unwrap();
        let e = cat.lookup(1, 0).unwrap();
        // One arg "month" -> index 0.
        let out = e.render(Languages::En, &["January".into()]);
        assert_eq!(out, "The month January!");
        assert!(matches!(out, Cow::Owned(_)));
    }

    #[test]
    fn multi_arg_sorted_indices() {
        // "current" < "total" alphabetically -> current=0, total=1. Caller must
        // pass args in that same sorted order.
        let bytes = encode_catalog(Languages::En, vec![plain(1, 0, "Page {current} of {total}")]);
        let cat = Catalog::new(&bytes).unwrap();
        let e = cat.lookup(1, 0).unwrap();
        let out = e.render(Languages::En, &[3i32.into(), 10i32.into()]);
        assert_eq!(out, "Page 3 of 10");
    }

    #[test]
    fn repeated_arg() {
        let bytes = encode_catalog(Languages::En, vec![plain(1, 0, "{x}-{x}")]);
        let cat = Catalog::new(&bytes).unwrap();
        let e = cat.lookup(1, 0).unwrap();
        assert_eq!(e.render(Languages::En, &["ab".into()]), "ab-ab");
    }

    #[test]
    fn escaped_braces() {
        let bytes = encode_catalog(Languages::En, vec![plain(1, 0, "{{not a var}} {x}")]);
        let cat = Catalog::new(&bytes).unwrap();
        let e = cat.lookup(1, 0).unwrap();
        let out = e.render(Languages::En, &[5i32.into()]);
        assert_eq!(out, "{not a var} 5");
    }

    #[test]
    fn plural_selection() {
        // Driver "n" is the only name -> index 0. Args pass the count at 0.
        let entry = EntrySpec {
            app_id: 2,
            variant: 0,
            forms: vec![
                FormSpec {
                    category: Plural::One,
                    template: "{n} apple".into(),
                },
                FormSpec {
                    category: Plural::Other,
                    template: "{n} apples".into(),
                },
            ],
            plural_driver: Some("n".into()),
        };
        let bytes = encode_catalog(Languages::En, vec![entry]);
        let cat = Catalog::new(&bytes).unwrap();
        let e = cat.lookup(2, 0).unwrap();
        assert_eq!(e.render(Languages::En, &[1i32.into()]), "1 apple");
        assert_eq!(e.render(Languages::En, &[3i32.into()]), "3 apples");
        assert_eq!(e.render(Languages::En, &[0i32.into()]), "0 apples");
    }

    #[test]
    fn plural_with_extra_arg() {
        // Names: {n, user} -> n=0, user=1. Driver "n" -> plural_arg_index 0.
        let entry = EntrySpec {
            app_id: 2,
            variant: 1,
            forms: vec![
                FormSpec {
                    category: Plural::One,
                    template: "{user} has {n} file".into(),
                },
                FormSpec {
                    category: Plural::Other,
                    template: "{user} has {n} files".into(),
                },
            ],
            plural_driver: Some("n".into()),
        };
        let bytes = encode_catalog(Languages::En, vec![entry]);
        let cat = Catalog::new(&bytes).unwrap();
        let e = cat.lookup(2, 1).unwrap();
        // args in sorted-name order: [n, user].
        assert_eq!(
            e.render(Languages::En, &[1i32.into(), "Sam".into()]),
            "Sam has 1 file"
        );
        assert_eq!(
            e.render(Languages::En, &[4i32.into(), "Sam".into()]),
            "Sam has 4 files"
        );
    }
}
