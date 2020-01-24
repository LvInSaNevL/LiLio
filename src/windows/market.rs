#[path = "style.rs"]
mod style;
use crate::market as marketData;
use crate::utils;

extern crate gdk;
extern crate gtk;
use gtk::prelude::*;
use gtk::*;

pub fn marketMenu() -> String {
	// Defining the root window the header bar
	let window = Window::new(WindowType::Toplevel);
	let res: Vec<i32> = utils::getResolution();

	// Setting the window name for the user and wm
	window.set_wmclass("lilio", "LiLio");
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
	scroll.set_margin_start(0);
	let appData = marketData::ReadMarket(true);
	style::stylizer("FC4A1F".to_string(), "AC0D57".to_string());

	let menuBox = gtk::ButtonBox::new(gtk::Orientation::Vertical);
	menuBox.set_spacing(buttonSize / 2);
	menuBox.set_margin_top(100);
	menuBox.set_margin_bottom(100);

	// Loops through the users market/device-list.json file to create the buttons they want
	for apps in appData {
		let newButton = Button::new_with_label(&apps.name);
		newButton.set_size_request(buttonSize, buttonSize);
		newButton.connect_clicked(move |_| {
			println!("{} from the market!", apps.name);
		});
		menuBox.pack_start(&newButton, false, false, 0);
	}

	scroll.add(&menuBox);
	window.add(&scroll);
	window.show_all();

	return "halt".to_string();
}
