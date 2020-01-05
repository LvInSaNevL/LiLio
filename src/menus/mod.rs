mod style;
use crate::market as marketData;

extern crate gdk;
extern crate gtk;
use gtk::prelude::*;
use gtk::*;

#[derive(Clone)]
pub struct App {
	pub window: Window,
}

#[derive(Clone)]
pub struct Header {
	pub container: HeaderBar,
}

// Creating the actual root window for LiLio
impl App {
	pub fn new(res: Vec<i32>) -> App {
		// Defining the root window the header bar
		let window = Window::new(WindowType::Toplevel);

		// Setting the window name for the user and wm
		window.set_wmclass("lilio", "LiLio");
		window.set_default_size(res[0], res[1]);
		window.set_decorated(false);

		// Exit button logic
		window.connect_delete_event(move |_, _| {
			main_quit();
			Inhibit(false)
		});

		// Return to the main program
		App { window }
	}
}

static mut MAIN: bool = true;
static mut SWITCH: bool = true;
pub fn menuManager(app: App, res: Vec<i32>) {
	let buttonSize = res[1] / 3;

	unsafe {
		loop {
			if SWITCH {
				println!("{}", SWITCH.to_string());
				if MAIN {
					println!("main");
					app.window.add(&mainMenu(buttonSize));
					app.window.show_all();
					SWITCH = false;
				} else {
					println!("market");
					app.window.add(&marketMenu(buttonSize));
					app.window.show_all();
					SWITCH = false;
				}
			} else {
				break;
			}
		}
	}
}

fn mainMenu(buttonSize: i32) -> gtk::ScrolledWindow {
	let scroll = gtk::ScrolledWindow::new(None, None);
	scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
	let appData = marketData::ReadMarket(false);
	style::stylizer("AC0D57".to_string(), "FC4A1F".to_string());

	let menuBox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
	menuBox.set_spacing(buttonSize / 2);
	menuBox.set_margin_start(100);
	menuBox.set_margin_end(100);

	// The market button always needs to be created so it goes in its own code block
	let marketButton = Button::new_with_label("Market");
	marketButton.set_size_request(buttonSize, buttonSize);
	marketButton.connect_clicked(move |_| unsafe {
		MAIN = false;
		SWITCH = true;
	});
	menuBox.pack_start(&marketButton, false, false, 0);

	// Loops through the users market/device-list.json file to create the buttons they want
	for apps in appData {
		let newButton = Button::new_with_label(&apps.name);
		newButton.set_size_request(buttonSize, buttonSize);
		newButton.connect_clicked(move |_| {
			println!("{}", apps.name);
		});
		menuBox.pack_start(&newButton, false, false, 0);
	}

	scroll.add(&menuBox);
	return scroll;
}

fn marketMenu(buttonSize: i32) -> gtk::ScrolledWindow {
	let scroll = gtk::ScrolledWindow::new(None, None);
	scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
	let appData = marketData::ReadMarket(true);
	style::stylizer("FC4A1F".to_string(), "AC0D57".to_string());

	let menuBox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
	menuBox.set_spacing(buttonSize / 2);
	menuBox.set_margin_start(100);
	menuBox.set_margin_end(100);

	// Loops through the global market/full-list.json file to create the buttons
	for apps in appData {
		let newButton = Button::new_with_label(&apps.name);
		newButton.set_size_request(buttonSize, buttonSize);
		newButton.connect_clicked(move |_| {
			println!("{}", apps.name);
		});
		menuBox.pack_start(&newButton, false, false, 0);
	}

	scroll.add(&menuBox);
	return scroll;
}
