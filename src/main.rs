#![allow(non_snake_case)]

mod market;
mod utils;
mod windows;
#[path = "windows/style.rs"]
mod style;
use std::process;

// Init point for LiLio
fn main() {
	utils::splashPrint();

	// Checks if the marketplace is up to date
	market::Download(
		"https://raw.githubusercontent.com/LvInSaNevL/LiLio_market/master/market_minified.json",
	)
	.expect(
		"Unable to download the most recent market data. Please check your network connection.",
	);

	// Inits GTK before rest of the codebase
	if gtk::init().is_err() {
		eprintln!("Failed to initialize GTK Application");
		process::exit(1);
	}

	// Inits the UI and widgets
	style::mainStylizer("FD698B".to_string(), "1D0740".to_string());
	windows::windowSelect("main".to_string());

	// Enter the GTK main event loop
	gtk::main();
}