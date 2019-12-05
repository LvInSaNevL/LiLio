mod ui_manager;
mod io_manager;
use gtk::*;
use std::process;
use std::env;

// Init point for LiLio
fn main() {
    io_manager::splashPrint();
    // Checks the arguments passed at the command line
    io_manager::flagParse(env::args().collect());

    // Inits GTK before rest of the codebase
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    // Inits the UI and widgets
    let app = ui_manager::App::new();
    app.window.show_all();

    // Enter the GTK main event loop
    gtk::main();
}