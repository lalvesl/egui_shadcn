# Rust i18n — compile-time catalogs, no HashMap

A from-scratch internationalization system built as a 4-crate Cargo workspace.
Translations are compiled into the binary as a packed byte buffer that is queried
with **binary search** — there is no `HashMap` anywhere on the hot path, so there
is no per-key heap fragmentation. Languages are **Cargo features**: a language is
in the binary only if its feature is on. On wasm you turn them all off and fetch
strings from a server instead.

```
i18n-format   dependency-free core: types + the binary catalog encoder/decoder.
              Used at COMPILE TIME by the macros and at RUNTIME by the facade.
i18n-macros   proc-macros: `traductions!` (declares an app enum + bakes catalogs)
              and `t!` (looks a string up for the current language).
i18n          runtime facade an app depends on: language registry, current-language
              state, the async `Source` fallback, and the lookup dispatcher.
example-app   a runnable demo with a mock egui `Ui` and a stub server `Source`.
```

## Build & run

```bash
cargo test                                   # all crates
cargo run -p example-app                     # default features: lang-en + lang-pt-br
cargo run -p example-app --no-default-features --features lang-en   # pt-BR now misses -> async fetch
cargo run -p example-app --no-default-features                      # wasm model: everything fetched
```

## How a translation is declared

```rust
i18n::traductions! {
    pub enum Calendar {
        January([En("January"), PtBr("Janeiro")]),
        Month([En("The month {month}"), PtBr("O mês {month}")]),   // placeholder
        Apples(plural {                                            // plural/singular
            one:   [En("{n} apple"),  PtBr("{n} maçã")],
            other: [En("{n} apples"), PtBr("{n} maçãs")],
        }),
    }
}
```

`traductions!` regenerates the enum **without** the `LanguagesWithValue` slices
(so `Calendar::January` is a plain unit variant) and implements the `Translate`
trait. `Translate::translation_id()` returns a `const` `u16` produced at compile
time by FNV-1a hashing the enum name with a constant salt.

> Design note: this is a **function-like** macro (`traductions! { ... }`), not the
> `#[i18n::traductions]` attribute from the original sketch. An attribute macro
> requires the item it decorates to already be valid Rust, but
> `January([En("January"), ...])` is not a valid enum variant (`En("January")`
> is not a type). A function-like macro receives the raw tokens, so it can accept
> the exact `[En(...), PtBr(...)]` DSL you wanted.

## How a translation is used

```rust
ui.label(t!(Calendar::January));
ui.label(t!(Calendar::Month, month = some_str));
ui.label(t!(Calendar::Apples, count = 5));            // picks the plural form
```

`t!` reads the current `Languages` value and calls the runtime, which returns a
`Cow<'static, str>` — borrowed (zero allocation) for plain strings, owned only
when a placeholder/plural actually has to be formatted. egui is **not** a
dependency: `t!` just yields a `Cow` and you hand it to `ui.label`, which accepts
anything `Into<String>`/`Into<WidgetText>`.

## The key & the binary format (no HashMap)

The composite key is your `U24`: the enum's `u16` app-id in the high bits and the
variant's `u8` (`Variant as u8`) in the low bits, packed into a `u32`:

```
key = (app_id << 8) | variant
```

Each (enum × language) pair is encoded at compile time into **one contiguous
`&'static [u8]`** living in `.rodata`:

```
header (16 bytes): magic b"I18N", version, language, key_count:u16,
                   records_offset:u32, strings_offset:u32
key table:         key_count × 12-byte records, SORTED by key, for binary search
                   [app_id:u16][variant:u8][flags:u8][offset:u32][len:u32]
strings region:    plain  -> raw UTF-8 template bytes
                   plural -> [form_count:u8] then [category:u8][len:u16][bytes]…
```

Lookup is: `match` on the language discriminant to pick the per-language set of
catalogs (no map), then binary-search the sorted key table inside the chosen
catalog. `flags` carries `F_ARGS` (has `{placeholder}`) and `F_PLURAL`; when
`F_ARGS` is clear the bytes are returned as `Cow::Borrowed` with no allocation.

> Serialization note: the on-disk widths are fixed (`u16`/`u32`) rather than the
> platform `usize` from the original sketch. A catalog is a portable artifact (it
> can be fetched over the wire on wasm), so fixed widths keep it stable across
> 32/64-bit and native/wasm. `key_count` is a `u16` because the variant index is
> a `u8` (≤ 256 variants per enum) — that is plenty per application enum.

## Feature-gating (what actually ends up in the binary)

For each language the macro emits the catalog bytes inside
`#[cfg(feature = "lang-xx")] const _: () = { … linkme registration … };`. The
`cfg` is evaluated in **your** crate, so your app's feature flags decide which
language bytes are compiled in. You can verify it:

```
strings target/debug/example-app | grep Janeiro   # present only with lang-pt-br
```

## The wasm / missing-language path

If the current language's feature was not compiled in (or you built with
`--no-default-features`, the wasm model), the lookup misses the embedded catalogs.
The runtime then:

1. calls the registered async `Source` (`notify_missing`) so your code can fetch
   `/i18n/{bcp47}/{app_id}/{variant}` from a server and later
   `install_runtime_catalog(...)` the bytes it gets back, and
2. returns the language's `fallback()` string (EnUs→En, PtBr→Pt) if available, or
   a visible `⟦app_id:variant⟧` marker so missing strings are obvious in the UI.

A fetched catalog uses the **exact same binary format**, so the server can ship
the very bytes the macro would have baked in.

## Languages

`Languages { En, EnUs, Pt, PtBr }` with stable discriminants `0,1,2,3`.
`Languages::parse(&str) -> Option<Languages>` is case-insensitive, accepts `-`
or `_`, and falls back region→base (`"en-GB"` → `En`). `LanguagesWithValue`
carries a `&'static str` per variant (`En(&'static str)`, …) and is the DSL the
`traductions!` arrays are written in.
