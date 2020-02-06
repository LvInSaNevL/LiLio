mod market;
mod style;
use crate::market as marketData;
use crate::utils;

extern crate gdk;
extern crate gtk;
use gtk::prelude::*;
use gtk::*;
use std::process::Command;

pub fn windowSelect(target: String) {
	match target.as_ref() {
		"halt" => println!("Base case, just ignore this"),
		"main" => mainCall(),
		"market" => marketCall(),
		_ => println!("Invalid window"),
	}
}

fn mainCall() {
	windowSelect(mainMenu());
}
fn marketCall() {
	windowSelect(market::marketMenu());
}

fn mainMenu() -> String {
	// Defining the root window the header bar
	let window = Window::new(WindowType::Toplevel);
	let res: Vec<i32> = utils::getResolution();

	// Setting the window name for the user and wm
	window.set_wmclass("lilio_market", "LiLio Market");
	window.set_default_size(res[0], res[1]);
	window.set_decorated(false);

	// Exit button logic
	window.connect_delete_event(move |_, _| {
		main_quit();
		Inhibit(false)
	});

	let buttonSize = res[1] / 3;
	let scroll = gtk::ScrolledWindow::new(None, None);
	scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
	let appData = marketData::ReadMarket(false);

	let menuBox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
	menuBox.set_spacing(buttonSize / 2);
	menuBox.set_margin_start(100);
	menuBox.set_margin_end(100);

	// The market button always needs to be created so it goes in its own code block
	let marketButton = Button::new_with_label("Market");
	marketButton.set_size_request(buttonSize, buttonSize);
	marketButton.grab_focus();
	marketButton.connect_clicked(move |_| {
		windowSelect("market".to_string());
	});
	menuBox.pack_start(&marketButton, false, false, 0);

	// Loops through the users market/device-list.json file to create the buttons they want
	for apps in appData {
		let newButton = Button::new_with_label(&apps.name);
		newButton.set_size_request(buttonSize, buttonSize);
		newButton.connect_clicked(move |_| {
			style::mainStylizer(apps.colorA.to_string(), apps.colorB.to_string());
			Command::new("sh")
				.arg("-c")
				.arg(format!(
					"chromium-browser --disable-notification --start-fullscreen --kiosk {}",
					&apps.file
				))
				.spawn()
				.expect("Failed to launch Chromium");
		});
		menuBox.pack_start(&newButton, false, false, 0);
	}

	scroll.add(&menuBox);
	window.add(&scroll);
	window.show_all();

	return "halt".to_string();
}
