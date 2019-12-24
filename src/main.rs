#![allow(non_snake_case)]

mod market;
mod ui_manager;
mod utils;
extern crate pipers;
use gtk::*;
use pipers::Pipe;
use std::io;
use std::process;
use std::str;

// Init point for LiLio
fn main() {
    utils::splashPrint();
    let mut resolution: Vec<i32> = utils::getResolution();

    // Checks if the marketplace is up to date
    //market::Download("https://raw.githubusercontent.com/LvInSaNevL/LiLio_market/master/market_minified.json");

    // Inits GTK before rest of the codebase
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    // Inits the UI and widgets
    let app = ui_manager::App::new(resolution.clone());
    ui_manager::mainMenu(app, resolution.clone());

    // Enter the GTK main event loop
    gtk::main();
}
