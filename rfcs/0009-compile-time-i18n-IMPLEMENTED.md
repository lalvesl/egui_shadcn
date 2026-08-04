# RFC 0009 — Compile-time i18n catalogs

|             |            |
| ----------- | ---------- |
| **Status**  | IMPLEMENTED |
| **Area**    | `i18n/*`   |
| **Created** | 2026-08-03 |
| **Updated** | 2026-08-03 |

## Summary

A from-scratch internationalization system: translations are declared next to the
enum that names them, baked into the binary as a packed byte buffer, and looked
up by **binary search** over a composite `u24` key. No `HashMap` on the hot path.
Languages are Cargo features. On wasm, catalogs ship as fetched assets instead.

## Motivation

egui applications repaint continuously, so a translation lookup runs on every
string, every frame. The usual `HashMap<String, String>` design pays hashing and
heap fragmentation for text that is fully known at compile time — and it links
every language into the binary whether or not the target uses them, which is
exactly the wrong trade for a wasm bundle.

The requirement was: declare a translation once, in one place, with placeholders
and plurals; pay nothing at runtime for languages that are not shipped; and keep
the call site as short as `t!(Calendar::January)`.

## Design

Four crates:

| Crate         | Role                                                                                |
| ------------- | ----------------------------------------------------------------------------------- |
| `i18n-format` | Dependency-free core: types plus the binary catalog encoder/decoder. Used at **compile time** by the macros and at **runtime** by the facade. |
| `i18n-macros` | `traductions!` (declares an app enum and bakes its catalogs) and `t!` (looks up a string for the current language). |
| `i18n`        | Runtime facade: language registry, current-language state, the async `Source` fallback, and the lookup dispatcher. |
| `example-app` | Runnable demo with a mock egui `Ui` and a stub server `Source`.                      |

Declaration and use:

```rust
i18n::traductions! {
    pub enum Calendar {
        January([En("January"), PtBr("Janeiro")]),
        Month([En("The month {month}"), PtBr("O mês {month}")]),
        Apples(plural(n) {
            one:   [En("{n} apple"),  PtBr("{n} maçã")],
            other: [En("{n} apples"), PtBr("{n} maçãs")],
        }),
    }
}

ui.label(t!(Calendar::January));
ui.label(t!(Calendar::Month, month = some_str));
ui.label(t!(Calendar::Apples, n = 5));
```

`traductions!` re-emits the enum *without* the language slices — `Calendar::January`
is a plain unit variant — and implements `Translate`, whose `translation_id()` is
a `const u16` produced at compile time by FNV-1a hashing the enum name with a
constant salt.

## Reference-level detail

- **Function-like, not attribute.** `#[i18n::traductions]` cannot work: an
  attribute macro requires the decorated item to already be valid Rust, and
  `January([En("January"), …])` is not a valid enum variant. A function-like
  macro receives raw tokens and can accept the DSL verbatim.
- **Key layout.** A composite `u24`: the enum's `u16` app-id in the high bits,
  the variant index in the low byte. Entries are sorted, so lookup is a binary
  search over a contiguous buffer — no hashing, no per-key allocation.
- **Return type** is `Cow<'static, str>`: borrowed (zero allocation) for plain
  strings, owned only when a placeholder or plural actually has to be formatted.
  egui is **not** a dependency — `t!` yields a `Cow` and the caller hands it to
  `ui.label`.
- **Positional arguments.** There is no `count` keyword; the plural driver is an
  ordinary named argument declared on the definition (`plural(n) { … }`) and
  passed like any other (`n = 5`). `t!` sorts the argument names it was given and
  emits values in that order; the catalog computed the same order by sorting the
  same names, so **placeholder names never travel in the binary**. Arguments bind
  by name regardless of order, so a translation may reorder them freely
  (`"{days} dias do mês {month}"`).
- **Languages are features.** `lang-en`, `lang-pt-br`, … A language is in the
  binary only if its feature is on; a missing language falls back to the async
  `Source`.
- **wasm.** `linkme` has no wasm backend, so the distributed-slice registration is
  compiled out on `wasm32` and the catalogs ship as generated assets under
  `wasm_assets/i18n/` (produced from the native binary by `nix build .#web`).
  Without them every string renders blank.
- **Consumer caveat.** `t!` / `traductions!` expand to absolute `::i18n::…`
  paths, so any crate that *invokes* them (e.g. `demo`) must depend on `i18n`
  directly — the `egui_sc::i18n` re-export only covers the runtime API.

## Drawbacks

- The current language is **global mutable state**. Tests that assert specific
  translations must serialize on a `Mutex` or they race under the parallel test
  runner — see [RFC 0010](0010-testing-and-e2e-strategy-IMPLEMENTED.md).
- Adding a language means adding a Cargo feature, which is a build-configuration
  change, not a data change. Translators cannot ship a language without touching
  the build.
- Positional arguments make the catalog format compact but opaque: an argument
  mismatch surfaces as a wrong substitution rather than a named error.

## Alternatives

- **`fluent` / `gettext`.** Rejected: runtime parsing, heap-heavy catalogs, and a
  dependency footprint that lands in the wasm bundle.
- **`HashMap<&str, &str>` keyed by string.** Rejected: hashing per lookup per
  frame, and the key strings themselves have to be in the binary.
- **Attribute macro form.** Rejected on the syntax grounds described above.

## Implementation status

- [x] Centralized language enum with locale-string parsing
- [x] `traductions!` DSL with placeholders and plurals
- [x] `Translate` trait with a compile-time FNV-1a `u16` app-id
- [x] Per-language Cargo features
- [x] `t!` bound to the current language, returning `Cow<'static, str>`
- [x] Packed contiguous catalog with binary-search lookup (no `HashMap`)
- [x] Positional argument indices — placeholder names stripped from the binary
- [x] wasm catalog assets + `linkme` excluded on `wasm32`
- [x] i18n applied across all components, charts and the demo
