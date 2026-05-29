# Todo

- Add in Example of buttons one with icon;

# Process

- Calendar with cell content, when select some day or a current day no centralized on the number but centralized by the cell, and this is the problem the cell has another think causing a decentralization of the number;

# Done

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

# Not so necessary

- Create another crate of demo-macro, with macro to "copy" code of implementation of component, and create a method to expose this code in demo ui, like a showroom of elements and the code;
