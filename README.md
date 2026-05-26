# egui-shadcn

Rust [egui](https://github.com/emilk/egui) implementation of [Shadcn/ui](https://ui.shadcn.com/) components. Not pixel-perfect, aesthetically faithful.

## Components

| Component | Notes                                                              |
| --------- | ------------------------------------------------------------------ |
| Button    | 6 variants (default, destructive, outline, secondary, ghost, link) |
| Input     |                                                                    |
| Label     |                                                                    |
| Badge     |                                                                    |
| Card      |                                                                    |
| Checkbox  |                                                                    |
| Radio     |                                                                    |
| Select    |                                                                    |
| Slider    |                                                                    |
| Switch    |                                                                    |
| Textarea  |                                                                    |
| Progress  |                                                                    |
| Separator |                                                                    |
| Avatar    |                                                                    |
| Alert     |                                                                    |
| Tabs      |                                                                    |
| Accordion |                                                                    |
| Dialog    |                                                                    |
| Tooltip   |                                                                    |

Accordion, Alert, Alert Dialog, Aspect Ratio, Avatar, Badge, Breadcrumb, Button Group, Calendar, Card, Carousel, Chart, Checkbox, Collapsible, Combobox, Command, Context Menu, Data Table, Date Picker, Dialog, Direction, Drawer, Dropdown Menu, Empty, Field, Hover Card, Input, Input Group, Input OTP, Item, Kbd, Label, Menubar, Native Select, Navigation Menu, Pagination, Popover, Progress, Radio Group, Resizable, Scroll Area, Select, Separator, Sheet, Sidebar, Skeleton, Slider, Sonner, Spinner, Switch, Table, Tabs, Textarea, Toast, Toggle, Toggle Group, Tooltip, Typography

## Running

Requires [Nix](https://nixos.org/) with flakes enabled.

```sh
nix develop
cargo run
```

## Tech Stack

- Rust
- [egui](https://github.com/emilk/egui) 0.31
- [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) 0.31
- Nix flakes

## Structure

```
src/
├── main.rs          — demo showcase app
├── theme.rs         — color tokens (Shadcn design tokens)
└── components/      — one file per component
```
