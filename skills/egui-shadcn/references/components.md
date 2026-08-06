# Component API reference

Every builder ends in `.show(...)`. Builder methods are omitted where they only
repeat the pattern (`.size(Size)`, `.width(f32)`, `.enabled(bool)`).

`ui` means `&mut Ui`; `ctx` means `&egui::Context`.

## Catalog

Every component, its constructor, its builder properties, and what `.show()`
gives back. Components marked **ctx** render at viewport level and take
`&egui::Context` instead of `&mut Ui`.

| Component | Constructor | Properties | `.show()` | Returns |
|---|---|---|---|---|
| `Accordion` | `new(id, title, &mut open)` | — | `(ui, content)` | `()` |
| `Alert` | `new(title)` | `description` `variant` | `(ui)` | `()` |
| `AlertDialog` **ctx** | `new(title, description, &mut open)` | `cancel_label` `confirm_label` `destructive` `width` | `(ctx, on_confirm)` | `bool` |
| `Avatar` | `new(initials)` | `color` `size` | `(ui)` | `()` |
| `Badge` | `new(label)` | `variant` `size` | `(ui)` | `()` |
| `Boxed` | `new()` | `fill` `corner_radius` `shadow` `accent` `padding` `padding_px` `margin` | `(ui, content)` | `InnerResponse<R>` |
| `Breadcrumb` | `new(&[&str])` | `separator` | `(ui)` | `Option<usize>` |
| `Breakpoint` | `current(ui)` | `is_mobile()` `is_narrow()` | — | enum |
| `Button` | `new(label)` | `variant` `size`(`ButtonSize`) `icon` `enabled` `corner_radius` | `(ui)` | `Response` |
| `ButtonGroup` | `new(&[&str])` | `selected` `variant` `size` | `(ui)` | `Option<usize>` |
| `Calendar` | `single(id, &mut Option<CalDate>)` · `range(id, &mut start, &mut end)` | `cell_content` `cell_height` `cell_width` `compact` | `(ui)` | `()` |
| `Card` | `new()` | `padding` `shadow` | `(ui, content)` | `()` |
| `Carousel` | `new(id, item_count)` | `height` `width` `loop_` | `(ui, item_fn)` | `usize` |
| `Chart` | `new(&datasets, &labels)` | `kind` `height` `width` `show_grid` `show_legend` | `(ui)` | `()` |
| `Checkbox` | `new(&mut bool)` | `label` `enabled` `size` | `(ui)` | `Response` |
| `Collapsible` | `new(id, trigger, &mut open)` | — | `(ui, content)` | `()` |
| `Combobox` | `new(id, &mut Option<usize>, &options)` | `placeholder` `width` `size` | `(ui)` | `bool` |
| `Command` **ctx** | `new(id, &groups, &mut open)` | `placeholder` `width` | `(ctx)` | `Option<(usize, usize)>` |
| `ContextMenu` | `new(id, &items)` | `width` | `(ui, area_fn)` | `Option<usize>` |
| `DataTable` | `new(id, &columns, &mut filter)` | `striped` `page_size` | `(ui, rows, body_fn)` | `Option<(usize, SortDir)>` |
| `DatePicker` | `new(id, &mut Option<CalDate>)` | `placeholder` `width` | `(ui)` | `bool` |
| `Dialog` **ctx** | `new(title, &mut open)` | `width` `header` | `(ctx, content)` | `()` |
| `Drawer` **ctx** | `new(title, &mut open)` | `height` `handle` | `(ctx, content)` | `()` |
| `DropdownMenu` | `new(id, trigger_label, &items)` | `width` `size` | `(ui)` | `Option<usize>` |
| `Grid` | `new()` | `min_col_width` `gap` | `(ui, count, item_fn)` | `()` |
| `HoverCard` | `new(id)` | `delay_frames` `width` | `(ui, trigger_fn, content_fn)` | `()` |
| `Icon` | `new(glyph)` | `size` `color` `clickable` | `(ui)` · `paint(ui, pos, align, color)` | `Response` |
| `Input` | `new(&mut String)` | `label` `placeholder` `password` `enabled` `icon_left` `width` `bordered` | `(ui)` | `Response` |
| `InputOtp` | `new(&mut String, digits)` | `separator_after` | `(ui)` | `Response` |
| `Label` | `new(text)` | `required` | `(ui)` | `()` |
| `Menubar` | `new(&menus)` | — | `(ui)` | `Option<(usize, usize)>` |
| `NavigationMenu` | `new(&items, active)` | `vertical` | `(ui)` | `Option<usize>` |
| `Pagination` | `new(current, total)` | `siblings` | `(ui)` | `Option<usize>` |
| `Popover` | `new(id)` | `width` | `(ui, trigger_fn, content_fn)` | `()` |
| `Progress` | `new(value)` | `height` | `(ui)` | `()` |
| `Radio<T: PartialEq>` | `new(&mut current, value)` | `label` `enabled` `size` | `(ui)` | `Response` |
| `Resizable` | `new(id)` | `dir` `initial_split` `min_size` `height` | `(ui, first_fn, second_fn)` | `()` |
| `Select` | `new(&mut Option<usize>, &options)` | `placeholder` `width` `size` | `(ui)` | `bool` |
| `Separator` | `horizontal()` · `vertical()` | `thickness` `length` | `(ui)` | `()` |
| `Sheet` **ctx** | `new(title, &mut open)` | `side` `width` | `(ctx, content)` | `()` |
| `Skeleton` | `new(w, h)` · `circle(size)` | `radius` | `(ui)` | `()` |
| `Slider` | `new(&mut f32, min, max)` | `step` `size` | `(ui)` | `Response` |
| `Spinner` | `new()` | `size` `thickness` | `(ui)` | `()` |
| `Switch` | `new(&mut bool)` | `label` `enabled` `size` | `(ui)` | `Response` |
| `Table` | `new(&columns)` | `striped` | `(ui, rows, body_fn)` | `()` |
| `Tabs` | `new(id, &labels, &mut current)` | — | `(ui, content_fn)` | `()` |
| `Textarea` | `new(&mut String)` | `label` `placeholder` `rows` `max_rows` `max_width` `scroll` `enabled` | `(ui)` | `Response` |
| `Toaster` **ctx** | `push(ctx, title, variant)` · `push_with_desc(ctx, title, desc, variant)` | — | `show(ctx)` once per frame | `()` |
| `Toggle` | `new(&mut bool, label)` · `custom(&mut bool)` | `icon` `enabled` `size` `corner_radius` `bordered` | `(ui)` · `show_with(ui, content)` | `Response` · `InnerResponse<R>` |
| `ToggleGroup<T: PartialEq + Clone>` | `new(&items, &mut selected)` | `enabled` `size` | `(ui)` | `()` |
| `Tooltip` | `new(text)` | — | `wrap(ui, content)` | `Response` |
| Typography | free functions | — | `heading1..4`, `lead_text`, `body_text`, `muted_text`, `small_text`, `code_text` | `()` |

### Enums

| Enum | Variants |
|---|---|
| `Size` | `Sm` `Default` `Lg` |
| `ButtonSize` | `Sm` `Default` `Lg` `Icon` |
| `ButtonVariant` | `Default` `Success` `Warning` `Destructive` `Outline` `Secondary` `Ghost` `Link` |
| `BadgeVariant` | `Default` `Secondary` `Destructive` `Outline` |
| `AlertVariant` | `Default` `Destructive` `Warning` |
| `ToastVariant` | `Default` `Success` `Warning` `Destructive` |
| `ButtonGroupVariant` | `Default` `Outline` |
| `Spacing` | `Xs` `Sm` `Md` `Lg` `Xl` `Xl2` `Xl3` |
| `SheetSide` | `Left` `Right` |
| `ResizeDir` / `Orientation` / `NavOrientation` | `Horizontal` `Vertical` |
| `ChartKind` | `Bar` `Line` |
| `SortDir` | ascending / descending |
| `DropdownItem` / `ContextItem` / `MenubarMenuItem` | `Item { label, [shortcut,] disabled }` · `Separator` |

### Item structs

```rust
NavItem         { label, icon: Option, badge: Option }
MenubarItem     { label, items: &[MenubarMenuItem] }
CommandGroup    { heading: Option, items: &[CommandItem] }
CommandItem     { label, description: Option, shortcut: Option, icon: Option }
TableColumn     { header, width: Option<f32> }
DataColumn      { header, width: Option<f32>, sortable: bool }
ToggleGroupItem { label, icon: Option }
ChartDataset    { label, values: &[f64], color: Option<Color32> }
```

## Forms and input

```rust
Input::new(&mut s)
    .label("Email").placeholder("m@example.com")
    .password(false).icon_left(icons::SEARCH)
    .width(240.0).bordered(true).enabled(true)
    .show(ui) -> Response

Textarea::new(&mut s)
    .label("Bio").placeholder("…").rows(4)
    .max_rows(10)        // auto-grow ceiling
    .max_width(400.0)
    .scroll(true)        // scroll instead of grow
    .show(ui) -> Response

InputOtp::new(&mut s, 6).separator_after(3).show(ui) -> Response

Checkbox::new(&mut checked).label("Accept").size(Size::Default).show(ui) -> Response
Switch::new(&mut on).label("Airplane mode").show(ui) -> Response
Toggle::new(&mut pressed, "Bold").icon(icons::FORMAT_BOLD)
    .bordered(true).corner_radius(cr).show(ui) -> Response

// Arbitrary widgets on the toggle surface. Content is laid out
// left-to-right, vertically centered in a Size::height row, and labels
// inherit the pressed/hover foreground. A click-sensing widget inside the
// content eats the click instead of flipping the toggle.
Toggle::custom(&mut pressed).bordered(true).show_with(ui, |ui| {
    ui.label(RichText::new(ICON_STAR).font(icon_font_id(16.0)));
    ui.label("Starred");
    Badge::new("12").variant(BadgeVariant::Secondary).show(ui);
}) -> InnerResponse<R>   // .response for clicks, .inner for the closure's value

// Radio binds by value: T: PartialEq
Radio::new(&mut current, MyEnum::A).label("Option A").show(ui) -> Response

// ToggleGroup binds by value: T: PartialEq + Clone
let items = [(Align::Left, ToggleGroupItem { label: "Left", icon: None })];
ToggleGroup::new(&items, &mut current).show(ui)

Slider::new(&mut value, 0.0, 100.0).step(5.0).show(ui) -> Response

Label::new("Email").required(true).show(ui)
```

## Selection

```rust
// Returns true when the selection changed.
Select::new(&mut Option<usize>, &["A", "B"]).placeholder("Pick…").show(ui) -> bool
Combobox::new("id", &mut Option<usize>, &["A", "B"]).show(ui) -> bool

ButtonGroup::new(&["Day", "Week"]).selected(Some(0))
    .variant(ButtonGroupVariant::Outline).show(ui) -> Option<usize>

DropdownMenu::new("id", "Open", &[
    DropdownItem::Item { label: "Edit", disabled: false },
    DropdownItem::Separator,
]).show(ui) -> Option<usize>

ContextMenu::new("id", &[
    ContextItem::Item { label: "Copy", shortcut: Some("⌘C"), disabled: false },
    ContextItem::Separator,
]).show(ui, |ui| ui.label("right-click me")) -> Option<usize>

Menubar::new(&[MenubarItem { label: "File", items: &[
    MenubarMenuItem::Item { label: "New", shortcut: Some("⌘N"), disabled: false },
]}]).show(ui) -> Option<(usize, usize)>   // (menu, item)
```

## Buttons

```rust
Button::new("Save")
    .variant(ButtonVariant::Default)   // Success Warning Destructive Outline Secondary Ghost Link
    .size(ButtonSize::Default)         // Sm Default Lg Icon  ← not `Size`
    .icon(icons::CHECK)
    .enabled(true)
    .corner_radius(cr)
    .show(ui) -> Response
```

## Layout and surfaces

```rust
Boxed::new()
    .fill(theme.card).corner_radius(cr).shadow(sh)
    .accent(true)                       // primary bottom border
    .padding(Spacing::Md)               // or .padding_px(16.0)
    .margin(Spacing::Sm)
    .show(ui, |ui| { … }) -> InnerResponse<R>

Card::new().padding(16.0).shadow(sh).show(ui, |ui| {
    card_header(ui, "Title", Some("Description"));
    card_footer(ui, |ui| { … });
});

Separator::horizontal().thickness(1.0).length(200.0).show(ui);
Separator::vertical().show(ui);

Spacing::Md.show(ui);                   // never ui.add_space

Grid::new().min_col_width(220.0).gap(12.0).show(ui, count, |ui, i| { … });

Resizable::new("id").dir(ResizeDir::Horizontal)
    .initial_split(0.3).min_size(80.0).height(300.0)
    .show(ui, |ui| { … }, |ui| { … });

Breakpoint::current(ui)                 // .is_mobile() / .is_narrow()
```

## Overlays — these take `ctx`, not `ui`

```rust
Dialog::new("Title", &mut open).width(480.0).header(true)
    .show(ctx, |ui| { … });

AlertDialog::new("Are you sure?", &mut open)
    .cancel_label("Cancel").confirm_label("Delete")
    .destructive(true).width(400.0)
    .show(ctx, || { /* on_confirm */ }) -> bool

Sheet::new("Title", &mut open).side(SheetSide::Right).width(360.0)
    .show(ctx, |ui| { … });

Drawer::new("Title", &mut open).height(300.0).handle(true)
    .show(ctx, |ui| { … });

Command::new("cmd", &groups, &mut open).placeholder("Type…").width(560.0)
    .show(ctx) -> Option<(usize, usize)>

Toaster::push(ctx, "Saved", ToastVariant::Success);
Toaster::push_with_desc(ctx, "Saved", "Your changes are live", ToastVariant::Default);
Toaster::show(ctx);   // once per frame, at the end of update()
```

`CommandGroup { heading: Option<&str>, items: &[CommandItem] }`,
`CommandItem { label, description, shortcut, icon }` — all `Option` but `label`.

## Popups anchored to a trigger — these take `ui`

```rust
Popover::new("id").width(280.0).show(ui,
    |ui| Button::new("Open").show(ui),      // trigger, must return Response
    |ui| { … },                             // content
);

HoverCard::new("id").delay_frames(20).width(300.0).show(ui,
    |ui| Button::new("@user").show(ui),
    |ui| { … },
);

Tooltip::new("Copy to clipboard").wrap(ui, |ui| Button::new("Copy").show(ui)) -> Response
```

## Navigation

```rust
Tabs::new("id", &["Account", "Password"], &mut current)
    .show(ui, |ui, index| { … });

NavigationMenu::new(&[NavItem { label: "Home", icon: None, badge: Some("3") }], active)
    .vertical().show(ui) -> Option<usize>

Breadcrumb::new(&["Home", "Docs"]).separator("/").show(ui) -> Option<usize>

Pagination::new(current, total).siblings(1).show(ui) -> Option<usize>

Accordion::new("id", "Section title", &mut open).show(ui, |ui| { … });
Collapsible::new("id", "Show more", &mut open).show(ui, |ui| { … });
```

## Data display

```rust
Table::new(&[TableColumn { header: "Name", width: Some(160.0) }])
    .striped(true)
    .show(ui, row_count, |i, row| {
        row.cell(|ui| { ui.label(names[i]); });
    });

DataTable::new("id", &columns, &mut filter_string)
    .striped(true).page_size(10)
    .show(ui, row_count, |i, row| { row.cell(|ui| { … }); })
    -> Option<(usize, SortDir)>    // clicked sort column

// DataColumn { header, width: Option<f32>, sortable: bool }

Progress::new(0.6).height(8.0).show(ui);
Skeleton::new(200.0, 16.0).radius(4.0).show(ui);
Skeleton::circle(40.0).show(ui);
Spinner::new().size(Size::Default).thickness(2.0).show(ui);
Avatar::new("LA").color(Color32::from_rgb(…)).size(Size::Lg).show(ui);
Badge::new("New").variant(BadgeVariant::Secondary).show(ui);
Alert::new("Heads up!").description("…").variant(AlertVariant::Warning).show(ui);

Carousel::new("id", item_count).height(240.0).width(400.0).loop_(true)
    .show(ui, |i, ui| { … }) -> usize
```

## Calendar

```rust
Calendar::single("id", &mut Option<CalDate>).show(ui);

Calendar::range("id", &mut start, &mut end)
    .compact()                       // one month + both arrows instead of two
    .show(ui);

Calendar::single("id", &mut date)
    .cell_height(52.0).cell_width(54.0)   // required when using cell_content
    .cell_content(|ui, date| { … })
    .show(ui);

DatePicker::new("id", &mut Option<CalDate>).placeholder("Pick a date")
    .width(240.0).show(ui) -> bool

CalDate::new(2026, 8, 3) / CalDate::today() / .next_month() / .prev_month()
```

> `cell_content` is dropped silently if the cell is too short. Always set
> `cell_height` when you use it — see RFC 0011, which is fixing this.

## Typography

Free functions, not builders:

```rust
heading1(ui, "…"); heading2(ui, "…"); heading3(ui, "…"); heading4(ui, "…");
lead_text(ui, "…"); body_text(ui, "…"); muted_text(ui, "…");
small_text(ui, "…"); code_text(ui, "…");
```

## Charts

Prefer `egui_charts` over the built-in `Chart`:

```rust
use egui_sc::egui_charts::{ChartWidget, ChartTheme};

let theme = ShadcnTheme::get(ui.ctx());
let ct = ChartTheme::from_primary(theme.primary, mode, harmony, series, dist);
```

Pass a fully transparent background when the chart already sits inside a `Card` —
`ChartWidget` then skips its own card and border.

The legacy built-in is `Chart::new(&datasets, &labels).kind(ChartKind::Bar)
.show_grid(true).show_legend(true).show(ui)` with
`ChartDataset { label, values: &[f64], color: Option<Color32> }`.
