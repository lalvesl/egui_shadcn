# Todo

# Process

# Done

- Create another example of range-calendar with only one calendar, not tow colendars;
- Calendar has small error in your component, the arrow to click to next month is not in the "end" and shows like more near to middle, the arrow to return months is on start and this is correct;
- Sidebar auto-scroll to active section only works after first navigation — fixed by calling `request_repaint()` when `sidebar_needs_scroll` is set, guaranteeing the sidebar sees the flag in the next frame even when the user stops scrolling.
- For the Demo, i need to create a paralles effect, i need to binding all components inside the unique and large scroll, the sidebar will works like a goto links;
- Command pallet, does not reusing components, the separator, input, cards, typography is not used;
- You removed the calendar with custom elements inside each day, add again example but add randon sumbers to be like prices;
- The Drawer component needs to reuse other components, like Boxed, because its padding is incorrect.
- Many components inside not reuse existing components; this is a major issue, check each component if it's using already builded component such as recreate from "zero";
- Fix the clippy warnings;
- Improve the demo showcase: group related components, rewrite the demo to use only one component per tab, ordered alphabetically, maximize reuse of the current component stack, and add component descriptions. Keep the overview section and update the count of currently implemented components (currently 56).
- Reuse components to build the demo UI: use Title and description typography, a Popover for the theme selector, standard Slider components, a secondary Button for "reset to zinc", and a Separator for the sidebar.
- Reuse components to build the demo UI. Each component tab should utilize standard headings, separator components, spacing, typography, and Boxed or Card components.
- The Alert Dialog does not reuse already created components like typography, buttons, spaces, and cards.
- Create a default size enum (like button sizes) and implement it for other components: Badge, Select, Slider, Spinner, Radio Group, Checkbox, Avatar, Switch, Toggle, Toggle Group, Button Group, Combobox, and Dropdown Menu.
- In the Breadcrumb example, clicking does not trigger the correct feedback message (likely a demo-only bug), and the custom separator does not trigger any action when clicked.
- Create a `Boxed` component (named `Boxed` instead of `Box` because Rust has a built-in `Box` type) which provides standard padding and margins, and supports dynamic children (which egui handles beautifully).
- Track down and replace all occurrences of `add_space` with the new Space component.
- Fix page transition issues when navigating "many pages" in the pagination example.
- The pagination component does not display the current page number.
- Create a Space component to standardize spacing using a default size enum. The enum should support rendering directly to UI and implement conversion (`From`/`Into`) to `f32` or other numeric types.
- Create a Separator component (supporting both vertical and horizontal layouts).
- Split the demo app from the main crate as it is extremely large.
- Add an icon button variant to the button examples.
- The Dialog does not close when clicking outside of it or pressing Escape.
- The Dialog's input field uses egui's text edit directly instead of reusing the custom Input component.
- The `nix run .#web` command does not compile or run in web mode.
- The Dialog's close icon does not display correctly, showing a small square instead.
- The Textarea can contain more text than its visible height. Add options to scroll or auto-grow along the X, Y, or both axes, and include these in the examples.
- Textareas share scroll positions with each other.
- The `build.rs` implementation is incorrect. The downloaded fonts should be saved inside the target folder (`OUT_DIR`) instead of directly in the project workspace.
- The icon registry is generated in the project workspace. Move it to the target folder and include it using `include!(concat!(env!("OUT_DIR"), "/filename.rs"))`.
- Allow adding custom fonts for all text, using the same strategy as the material icons font.
- Use Nerd Fonts in the demo.
- Remove the dependency on the `assets` folder in `index.html`. Everything should be automated via `build.rs`.
- Continue implementing the remaining components listed in `README.md`, update their progress one by one, and ensure they are all added to the demo.
- Icons are extremely difficult to use. Create an easier way to render them by only passing the context and UI, while still allowing custom sizes and colors, with conventional defaults.
- In the Calendar with custom cell content, selecting a day (or highlighting the current day) does not center the number. The styling elements inside the cell cause the text to be decentralized.
- Reimplement Toggle Group to align closer with the new Toggle design.

# Not so necessary

- Create a separate `demo-macro` crate with a macro to extract/copy the component implementation code, and expose it in the demo UI to showcase both the component and its source code.
- better i18n implementation, only rust code, imagine, in another crate describe an enum with each Languages{ En, EnUs, Pt,
  PtBr} create implementation function recing &str of some language and can return some one Languages::ENUM_ELEMENT,
  and create the type LanguagesWithValue, enum with LanguesWithValue::ENUM(static'&str), create another enums to can
  sparete by application, or something like this, for example #[i18n::traductions]EnumForCalendarThisIsSomeApplication{January([En("January"), PtBr("Janeiro")]),
  .....antoher_things_of_this_application}, this regenerate this enum removing the LanguagesWithValues list (static
  slice) and generate the implementation of trait Translate, this has a method translation_id, return a const inline
  value u16 generated at compile time hashing the name of enum for application with constant salt inside of
  procedure-macro, the translations can be add inside binary depending of the features inside the crate, each language is
  a feature, to use transaltion there's a another procedude-macro "t!" this macro generate the code to bindings
  automatically for the ui-context of egui module and take the current language selected from enum Languages, and if the
  language is selected at compile time by feature, automatically access the value from function to fallback defined by
  implementation in Language Enum (the function to recive a possible language), if the language not define execute
  another async function to request this language or your fallback, how the key works in this case, the u16 constant
  value are the first key and the ApplicationEnum(in this example is EnumForCalendarThisIsSomeApplication) can be
  transformed in u8 using enum::January as u8, this compose the key "U24"(u16,u8) this need to storaged in unique
  continuos struct of Vec<u8> and the format is {quantity_of_keys: usize, keys: [Key (is a element with (u16,u8)
  ordenated by value to use some binary-search strategy, u8 ] and store inside of HashMap<[u16,u8],String>
