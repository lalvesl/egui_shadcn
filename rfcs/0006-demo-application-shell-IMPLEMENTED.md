# RFC 0006 — Demo application shell

|             |          |
| ----------- | -------- |
| **Status**  | IMPLEMENTED |
| **Area**    | `demo`   |
| **Created** | 2026-08-03 |
| **Updated** | 2026-08-03 |

## Summary

Split the showcase app into its own crate and give it a single-scroll shell: one
long scroll view containing every section, with the sidebar acting as anchor
links into it. The demo is built *out of the library it demonstrates*.

## Motivation

The demo is the project's primary artifact — the [live
site](https://lalvesl.github.io/egui_shadcn/) is how anyone evaluates the
library — and it had three problems:

1. **It lived in the library crate.** Its size dominated build times and its
   dependencies leaked into consumers of the component library.
2. **Tab-per-section navigation** hid most of the library. Nobody clicked
   through 20 tabs; the breadth of the component set was invisible.
3. **It did not eat its own dog food.** The command palette, the theme picker
   and the sidebar were hand-rolled egui, so the demo failed to demonstrate the
   very components it was shipping.

## Design

- **Crate split.** `demo/` is a separate workspace member depending on `egui_sc`
  (plus `i18n` directly — the `t!` macros expand to absolute `::i18n::…` paths,
  so the `egui_sc::i18n` re-export alone is not enough).
- **One scroll view.** All sections render into a single `ScrollArea`; the
  sidebar scrolls to a section rather than swapping the content. Scrolling
  therefore surveys the whole library, which is the point of a showcase.
- **One component per tab, alphabetically ordered**, each with a written
  description — the demo doubles as the component catalog.
- **Built from the library.** Typography for titles and descriptions, `Popover`
  for the theme selector, `Slider` for the hue controls, a secondary `Button`
  for "reset to zinc", `Separator` in the sidebar, and `Input` / `Card` /
  `Separator` / `Typography` inside the command palette.

## Reference-level detail

- `demo/src/app/sections/` — one module per component group (`buttons`,
  `content`, `feedback`, `forms`, `media`, `navigation`, `overlays`,
  `overview`).
- **Sidebar auto-scroll fix.** Scrolling the sidebar to the active section only
  worked after the first navigation: the flag was set during a frame that was
  already being laid out, and with no further input no repaint followed.
  `request_repaint()` is now called whenever `sidebar_needs_scroll` is set, so
  the next frame observes it even when the user has stopped interacting.
- **Toolbar alignment.** The top row's height is pinned
  (`set_min_height(36.0)`) *before* items are added, so the hamburger toggle and
  the dark/light icon center against the taller `Select` and heading instead of
  floating above them.
- **Breadcrumb feedback.** Clicks now report the correct index, and a custom
  separator is itself click-responsive.
- **Pagination.** Fixed page transitions across multiple pages and surfaced the
  current page number.
- `DemoApp::show(ui)` is split out of `eframe::App::update` so tests can step the
  real app headlessly — see [RFC 0010](0010-testing-and-e2e-strategy-IMPLEMENTED.md).

## Drawbacks

- One scroll view means every section lays out every frame. egui's immediate
  mode makes this cheap today, but it scales linearly with the component count.
- Anchor-style navigation makes deep-linking to a section a scroll-offset
  problem rather than a routing problem.

## Alternatives

- **Keep tab-per-section.** Rejected: it hid the library's breadth, which is the
  one thing the demo exists to show.
- **Keep the demo in the library crate.** Rejected: build time and dependency
  leakage onto every consumer.

## Implementation status

- [x] Demo split into its own crate
- [x] Single unified scroll view with sidebar anchor links
- [x] Sidebar auto-scroll fixed via `request_repaint()`
- [x] Toolbar row height pinned before item insertion
- [x] Command palette rebuilt on `Separator` / `Input` / `Card` / `Typography`
- [x] Sections grouped, one component per tab, alphabetical, with descriptions
- [x] Overview component count updated (56)
- [x] Breadcrumb click feedback + responsive custom separator
- [x] Pagination page transitions and current-page display
- [x] Icon button variant added to the button showcase
