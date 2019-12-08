#![allow(non_snake_case)]

mod ui_manager;
mod market;
use gtk::*;
use std::process;

// Init point for LiLio
fn main() {
    let splash_array = [
    "  | |      (_)    | |      (_)     ___",   
    "  | |__    | |    | |__    | |    / _ \\",  
    "  |____|  _|_|_   |____|  _|_|_   \\___/ ", 
    "_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|", 
    "\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-' "
    ];

    for line in splash_array.iter() {
        println!("{}", line)
    }

    // Checks if the marketplace is up to date
    market::Downloader("https://raw.githubusercontent.com/LvInSaNevL/LiLio_market/master/market.json", "market.json");

    // Inits GTK before rest of the codebase
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    // Inits the UI and widgets
    let app = ui_manager::App::new();
    app.window.show_all();

    // Creates the buttons
    let child_padding = 0;
    let information_label = Label::new("Specific Information: ");

    let horizontal_box = Box::new(Orientation::Horizontal, child_padding);

    horizontal_box.pack_start(&information_label, false, false, 0);

    // Enter the GTK main event loop
    gtk::main();
}