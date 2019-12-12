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
    market::Download("https://raw.githubusercontent.com/LvInSaNevL/LiLio_market/master/market_minified.json");
    market::ReadMarket(true);

    // Inits GTK before rest of the codebase
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    // Inits the UI and widgets
    let app = ui_manager::App::new();

    
    
    let button = Button::new_with_label("Stadia");
    button.connect_clicked(|_| {
        println!("Stadia");
    });
    app.window.add(&button);

    app.window.show_all();

    // Enter the GTK main event loop
    gtk::main();
}