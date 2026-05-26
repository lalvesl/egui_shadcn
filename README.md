# egui-shadcn

Rust [egui](https://github.com/emilk/egui) implementation of [Shadcn/ui](https://ui.shadcn.com/) components. Not pixel-perfect, aesthetically faithful.

## Components

| Component       | Status | Notes                                                              |
| --------------- | ------ | ------------------------------------------------------------------ |
| Accordion       | 🚧     | Collapsible sections                                               |
| Alert           | 🚧     | default / destructive / warning / variants                         |
| Alert Dialog    | 🚧     |                                                                    |
| Aspect Ratio    | 🚧     |                                                                    |
| Avatar          | 🚧     | Initials fallback, no image assets                                 |
| Badge           | 🚧     |                                                                    |
| Breadcrumb      | 🚧     |                                                                    |
| Button          | 🚧     | 6 variants (default, destructive, outline, secondary, ghost, link) |
| Button Group    | 🚧     |                                                                    |
| Calendar        | 🚧     |                                                                    |
| Card            | 🚧     | CardHeader, CardContent, CardFooter                                |
| Carousel        | 🚧     |                                                                    |
| Chart           | 🚧     |                                                                    |
| Checkbox        | 🚧     |                                                                    |
| Collapsible     | 🚧     |                                                                    |
| Combobox        | 🚧     |                                                                    |
| Command         | 🚧     |                                                                    |
| Context Menu    | 🚧     |                                                                    |
| Data Table      | 🚧     | Sorting, filtering, pagination                                     |
| Date Picker     | 🚧     |                                                                    |
| Dialog          | 🚧     | Modal overlay                                                      |
| Drawer          | 🚧     |                                                                    |
| Dropdown Menu   | 🚧     |                                                                    |
| Empty           | 🚧     |                                                                    |
| Field           | 🚧     |                                                                    |
| Hover Card      | 🚧     |                                                                    |
| Input           | 🚧     |                                                                    |
| Input Group     | 🚧     |                                                                    |
| Item            | 🚧     |                                                                    |
| Kbd             | 🚧     |                                                                    |
| Label           | 🚧     |                                                                    |
| Menubar         | 🚧     |                                                                    |
| Navigation Menu | 🚧     |                                                                    |
| Pagination      | 🚧     |                                                                    |
| Popover         | 🚧     |                                                                    |
| Progress        | 🚧     |                                                                    |
| Radio Group     | 🚧     |                                                                    |
| Resizable       | 🚧     |                                                                    |
| Scroll Area     | 🚧     |                                                                    |
| Select          | 🚧     | Dropdown                                                           |
| Separator       | 🚧     | Horizontal / vertical                                              |
| Sheet           | 🚧     |                                                                    |
| Sidebar         | 🚧     |                                                                    |
| Skeleton        | 🚧     |                                                                    |
| Slider          | 🚧     |                                                                    |
| Spinner         | 🚧     |                                                                    |
| Switch          | 🚧     | Toggle                                                             |
| Table           | 🚧     |                                                                    |
| Tabs            | 🚧     |                                                                    |
| Textarea        | 🚧     | Multiline                                                          |
| Toast           | 🚧     |                                                                    |
| Toggle          | 🚧     |                                                                    |
| Toggle Group    | 🚧     |                                                                    |
| Tooltip         | 🚧     | On hover                                                           |
| Typography      | 🚧     |                                                                    |

> ✅ Done &nbsp; 🚧 In development

## Running

Requires [Nix](https://nixos.org/) with flakes enabled.

```sh
nix develop
cargo run
```

## Tech Stack

- Rust
- [egui](https://github.com/emilk/egui) 0.34
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) 0.34
- Nix flakes

## Structure

```
src/
├── main.rs          — demo showcase app
├── theme.rs         — color tokens (Shadcn design tokens)
└── components/      — one file per component
```
