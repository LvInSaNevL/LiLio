#![allow(non_snake_case)]

mod ui_manager;
mod market;
extern crate pipers;
use gtk::*;
use pipers::Pipe;
use std::process::{Command, Stdio};
use std::process;
use std::io;
use std::str;

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

    // Gets the screen dimensions
    // xrandr | grep -w connected  | awk -F'[ +]' '{print $3}'
    let resOut = Command::new("xrandr")
                    .output()
                    .unwrap();
    let rawRes = String::from_utf8(resOut.stdout).unwrap();
    let displayIndex = rawRes.find("current ").unwrap() + 8;
    let formattedRes = &rawRes[displayIndex..(displayIndex+11)];

    println!("{:?}", formattedRes.to_string());

    // Checks if the marketplace is up to date
    market::Download("https://raw.githubusercontent.com/LvInSaNevL/LiLio_market/master/market_minified.json");

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