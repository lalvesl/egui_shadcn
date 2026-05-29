# Todo

- The Alert Dialog does not re use components already created like typography, buttons, spaces and cards;
- Create default enum for size like buttons size and implement for another components like badge, select, slider, spinner, Radio Group, Checkbox, Avatar, switch, toggle, toggle group, Button Group, Combobox, Drop down menu;

- Many components does not reuse components, that's a big problem;
- In the Breadcrumb example, does not show currecly the message when click, but i belive this error it's only for demo app, the custom separator does not show nothing when click;

- Create component Boxed (not Box because Rust already have Box structure), this is a simple box with standart padding and margin, yes this component need to recive dynamic childs internally, egui already supports awesome this part;

# Process

# Done

- Now i need to track every time executed add_space replace by new space component;
- Problem of transition of "Many pages" in pagination example;
- The pagination does not show the number inside of current page;
- Create component to standartization of spaces with default sizes that a enum and this enum can have the function to show reciving the ui and another From to execute into to automatically transform in f32 or another types of numbers;
- Create component separator (vertical and horizontal);
- Split the app from demo, extreme huge;
- Add in Example of buttons one with icon;
- The dialog not close if click out or press esc;
- In the dialog, contains an input of element, this input not using the component, only diracly egui input;
- The "nix run .#web" does not compiling and running for web mode;
- The icon of close dialog not showing correcly, show a small square;
- The textarea can write more text than your content, create option to scroll or grow to x or y or bolf, and add in example;
- The textarea share position of scroll between others textareas
- The implementation of @build.rs is wrong, the location to download fonts need's to be inside target folder this is wrong, not in codespace;
- The registry of icons is inside of codespace move to target and use include!(concat!(env!("OUT_DIR"), "/filename.rs"));
- Add possibility to add another font for all texts, with the same strategy of material icons font;
- In demo, use NerdFonts;
- Remove the necessity of assets folder from @index.html, everythink this need to be automated by build.rs;
- Continue of implementation of next components, read @readme.md and update one by one the progress, remember to add all components in demo;
- Icons is extreme difficult to use, create elements more easy only passing the context and UI, ofcourse with possibility to passing size and another color, but default is conventional to use;
- Calendar with cell content, when select some day or a current day no centralized on the number but centralized by the cell, and this is the problem the cell has another think causing a decentralization of the number;
- Reimplement Toggle group to be near of new toggle;

# Not so necessary

- Create another crate of demo-macro, with macro to "copy" code of implementation of component, and create a method to expose this code in demo ui, like a showroom of elements and the code;
