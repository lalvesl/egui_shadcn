# RFC 0012 — Demo source-snippet macro crate

|             |                    |
| ----------- | ------------------ |
| **Status**  | PLANNED-LOW              |
| **Area**    | `demo`, new crate  |
| **Created** | 2026-08-03         |
| **Updated** | 2026-08-03         |

## Summary

A `demo-macro` crate exposing a macro that captures the source text of the code
it wraps, so each demo section can display the exact snippet that produced the
widget above it — without maintaining a second, hand-written copy of that code.

## Motivation

Shadcn's site shows a component and, right beneath it, the code that renders it.
That pairing is the whole reason the site is useful for evaluation. The demo
currently shows only the rendered component, so a visitor who likes a widget has
to go find its call site in the repository.

The naive fix — paste the snippet into a string literal next to the code — is the
worst option available: two copies that drift the moment either is edited, and
nothing detects the drift. A macro that captures the tokens it wraps makes the
snippet unable to diverge, because there is only one source of truth.

## Design

```rust
demo_macro::with_source!(ui, {
    Button::new("Click me")
        .variant(ButtonVariant::Destructive)
        .show(ui);
});
```

The macro expands to:

1. the block, executed verbatim; and
2. a `&'static str` of the block's source, handed to a code viewer that renders
   it in a `Card` with a copy button.

The captured text must be **formatted source**, not `stringify!` output —
`stringify!` collapses whitespace and produces a single unreadable line, which
defeats the purpose.

## Reference-level detail

Two viable capture strategies, to be decided during implementation:

| Strategy | How | Trade-off |
| -------- | --- | --------- |
| **`proc_macro::Span::source_text`** | Ask the compiler for the original text of the span. | Exact original formatting, no file I/O. `Span::source_text` is stable, but returns `Option` and may yield `None` under some expansion contexts — needs a fallback. |
| **Read the file at compile time** | Use `Span::file()`/`line()` (or `file!`/`line!`) plus `include_str!` and slice the range. | Robust and simple, but the byte range must be recovered from token spans, and the whole file lands in the binary unless the slice is done at compile time. |

Either way the crate is `proc-macro = true` and must not pull the demo's runtime
dependencies into the build graph.

Rendering side: a `CodeBlock` view in the demo built on `Card` + the existing
`code` typography helper ([RFC 0002](0002-sizing-and-spacing-primitives-IMPLEMENTED.md),
[RFC 0006](0006-demo-application-shell-IMPLEMENTED.md)). Syntax highlighting is explicitly
out of scope for a first cut — monospace with correct indentation is already the
bulk of the value.

## Drawbacks

- A proc-macro crate is a compile-time dependency on `syn`/`proc-macro2` for what
  is, in the end, a documentation feature.
- Snippets are captured verbatim, including demo-specific state plumbing
  (`&mut self.some_field`), which is noise to a reader who wants the component
  API. A `# hidden line` convention like rustdoc's may be needed.
- Every captured snippet is a `&'static str` in the binary — measurable in a wasm
  bundle that [RFC 0008](0008-build-time-asset-pipeline-IMPLEMENTED.md) works to keep small.

## Alternatives

- **Hand-written snippet strings.** Rejected: guaranteed drift.
- **Link each section to its source on GitHub.** Much cheaper and loses the
  side-by-side reading that makes the pattern work. A reasonable interim step.
- **Extract snippets in a build script by parsing the section files.** Rejected:
  a fragile source-parsing step outside the compiler, for no gain over a macro.

## Unresolved questions

- Which capture strategy survives contact with `Span::source_text` returning
  `None`?
- Should the snippet include the surrounding `Card`/layout code, or only the
  component call? Only the component call is the useful answer, and it implies a
  hidden-line convention.
- Is the wasm size cost acceptable, or should snippets be a Cargo feature that
  the web build turns off?

## Implementation status

- [ ] `demo-macro` crate skeleton (`proc-macro = true`)
- [ ] `with_source!` capturing formatted source text
- [ ] `CodeBlock` view with copy-to-clipboard
- [ ] Applied to one section as a trial, then rolled out
- [ ] Wasm bundle size measured before and after
