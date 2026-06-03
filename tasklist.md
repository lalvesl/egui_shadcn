# Todo

- In demo, the icon to switch between light and dark mode, is not centralized in line with another elements, also the sidebar toggle too, it's more above.
- Calendar, when select range, after clicked in the first day automatically when hovering another days make "auto selected", only a visual trick for hovering;
  ** Animations, there's no animations for this application
  ** Accordion, can have animation to slide down and up;
  ** Dialog, to show up and goes down;
  ** Carousel, makes slide swap between right and left depending the what to go to;

- Replace the demo chart to use egui_charts crate in this directory, add bar and line charts equals already implemented in demo, don't forgot to pass the "primary" color to charts context, to automatically change;

-

# Process

_(No active tasks)_

# Done

### Components & Layout

- **Boxed Component**: Created a `Boxed` component (named `Boxed` instead of `Box` to avoid conflict with Rust's built-in `Box` type) which provides standard padding and margins, and supports dynamic children.
- **Space Component**: Created a `Space` component to standardize spacing using a default size enum. The enum supports rendering directly to the UI and implements conversion (`From`/`Into`) to `f32` and other numeric types.
- **Space Component Integration**: Tracked down and replaced all occurrences of `add_space` with the new `Space` component.
- **Separator Component**: Created a `Separator` component supporting both vertical and horizontal layouts.
- **Size Enum**: Created a default size enum (like button sizes) and implemented it across other components: `Badge`, `Select`, `Slider`, `Spinner`, `RadioGroup`, `Checkbox`, `Avatar`, `Switch`, `Toggle`, `ToggleGroup`, `ButtonGroup`, `Combobox`, and `DropdownMenu`.
- **Component Reuse**: Addressed components that were not reusing existing UI elements; checked each component to ensure it leverages already built primitives (e.g., `Boxed`, `Space`, `Separator`, `Card`, and `Typography`) rather than recreating them from scratch.
- **Alert Dialog**: Refactored the `AlertDialog` to reuse existing typography, button, space, and card components.
- **Dialog Improvements**:
  - Ensured the `Dialog` closes when clicking outside of it or pressing the `Escape` key.
  - Refactored the `Dialog` close icon to render correctly instead of displaying as a small square.
  - Updated the `Dialog` input fields to reuse the custom `Input` component instead of egui's text edit directly.
- **Drawer Component**: Refactored the `Drawer` component to reuse other components (like `Boxed`) to fix incorrect padding.
- **Textarea Enhancements**:
  - Allowed `Textarea` components to contain more text than their visible height by adding options to scroll or auto-grow along the X, Y, or both axes, and included these in the examples.
  - Fixed a bug where multiple `Textarea` components incorrectly shared scroll positions.
- **Toggle Group**: Reimplemented the `ToggleGroup` component to align closely with the new `Toggle` component design.

### Calendar & Date Picker

- **Single Range Calendar**: Added a range-calendar example using only a single calendar component (with navigation arrows on both sides) rather than two side-by-side calendars.
- **Calendar Layout Error**: Fixed a small layout bug where the next-month arrow was not aligned to the end (right) but appeared closer to the middle. The previous-month arrow was correctly aligned to the start (left).
- **Custom Cell Elements**: Added back the calendar example with custom elements inside each day, showing randomized values representing prices.
- **Calendar Number Centering**: Fixed an alignment issue in the custom-cell calendar where selecting a day or highlighting the current day did not center the number because of interior styling elements.

### Demo Application

- **Unified Scrolling UI**: Created a parallel/smooth scroll effect for the demo by binding all components inside a single, large scroll view, where the sidebar acts as anchor links.
- **Sidebar Auto-Scroll**: Fixed a bug where sidebar auto-scrolling to the active section only worked after the first navigation. This was resolved by calling `request_repaint()` when `sidebar_needs_scroll` is set, ensuring that the repaint loop sees the flag in the next frame even when the user stops scrolling.
- **Command Palette**: Refactored the command palette to reuse existing separator, input, card, and typography components.
- **Breadcrumb Feedback**: Fixed a bug in the breadcrumb example where clicking did not trigger the correct feedback message, and ensured the custom separator is responsive to clicks.
- **Demo Showcase Improvements**:
  - Grouped related components, rewrote the demo to use only one component per tab, ordered them alphabetically, maximized reuse of the current component stack, and added component descriptions.
  - Updated the count of currently implemented components in the overview section (to 56).
- **Demo UI Components**: Reused components to build the demo UI (e.g., using Title and Description typography, a `Popover` for the theme selector, standard `Slider` components, a secondary `Button` for 'reset to zinc', and a `Separator` for the sidebar).
- **Crate Decoupling**: Split the demo application into a separate crate from the main library due to its large size.
- **Button Examples**: Added an icon button variant to the button showcase.
- **Pagination Component**:
  - Fixed page transition issues when navigating through multiple pages in the pagination example.
  - Updated the pagination component to display the current page number.

### Build, Infrastructure & Assets

- **Clippy Warnings**: Fixed various Cargo Clippy warnings.
- **Build.rs Improvement**: Corrected the `build.rs` implementation so downloaded fonts are saved inside the build target folder (`OUT_DIR`) instead of polluting the project workspace directory.
- **Icon Registry**: Moved the generated icon registry from the workspace to the `OUT_DIR` folder, referencing it via `include!(concat!(env!("OUT_DIR"), "/filename.rs"))`.
- **Assets Automation**: Removed the dependency on the `assets` folder in `index.html` by automating asset integration through `build.rs`.
- **Font Support**:
  - Allowed custom fonts for all text, using the same strategy as the material icons font.
  - Integrated Nerd Fonts in the demo.
- **Easy Icon API**: Simplified icon rendering by creating a helper that only requires the context and UI, while supporting optional custom sizes, colors, and sensible defaults.
- **Nix Web Mode**: Fixed the `nix run .#web` command so it compiles and runs in web mode successfully.
- **Component Implementations**: Continued implementing the remaining components listed in `README.md`, updating progress, and ensuring they are showcased in the demo.

### Testing & E2E

- **Unit & interaction tests**: Added a headless `egui::Context` test harness (`egui_components/tests/common`) — no `egui_kittest`, since components paint directly. Covers every component rendering across variants/sizes/light+dark (`render_simple`, `render_complex`), behavioural clicks/drags/typing (`interaction`), and theme/token math (`theme`).
- **Chart tests**: `egui_charts` render-smoke for every chart kind (bar/line/scatter/pie/gauge/funnel/empty) in both modes, plus palette/theme assertions.
- **Example smoke tests**: `DemoApp` and the chart `GalleryApp` update loops are split out of `eframe::App::ui` (`show` / `render`) and stepped frame-by-frame headlessly over every section / chart kind / locale; the `i18n` example binary is run as part of the gate.
- **Rust web e2e (replaced Python Playwright)**: New standalone `e2e/` crate drives headless Chromium over CDP via `chromiumoxide`; self-serves `demo/dist`, scans the console for crash signatures, and screenshots the canvas asserting color diversity (not just non-background pixels). Deleted `e2e/test.py`.
- **Flake**: `nix run .#e2e` now runs tests → example execution → WASM build → browser test (Python-free); added `nix run .#test`. Provides `pkgs.chromium`; `E2E_CHROME` overrides.
- **Flaky-test fix**: serialized i18n language-dependent tests (global state) behind a `Mutex` so the parallel runner is deterministic.

# Not so necessary / Future Ideas

- **Demo Macro Crate**: Create a separate `demo-macro` crate exposing a macro to extract/copy a component's implementation code and display it inside the demo UI.
- **Advanced Rust i18n System**:
  - Define a centralized `Languages` enum (e.g., `En`, `EnUs`, `Pt`, `PtBr`) with a helper to parse locale strings.
  - Create a translation macro, e.g. `#[i18n::translations] EnumForCalendar { January([En("January"), PtBr("Janeiro")]), ... }`, that generates a compile-time representation of translatable items.
  - The macro should generate a `Translate` trait implementation where each item gets a unique `u16` ID generated at compile time by hashing the application enum's name with a constant salt.
  - Compile translations conditionally using crate features (one feature per language).
  - Implement a `t!` macro to automatically bind translations to the egui UI context based on the currently selected language.
  - Optimize memory usage by packing the translation key (combining the `u16` ID and a `u8` enum variant) and storing them in a contiguous `Vec<u8>` or `HashMap<[u8; 3], String>`, looking up entries with a binary-search lookup strategy.
