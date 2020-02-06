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
	window.set_wmclass("lilio_market", "LiLio Market");
	window.set_default_size(res[0], res[1]);
	window.set_decorated(false);

	// Exit button logic
	window.connect_delete_event(move |_, _| {
		main_quit();
		Inhibit(false)
	});

	let flow = gtk::FlowBox::new();
	let appData = marketData::ReadMarket(true);
	style::mainStylizer("AC0D57".to_string(), "FC4A1F".to_string());

	let stadiaData = &appData[0];
	let newFixed = gtk::Fixed::new();

	let newTitle = Label::new(None);
	newTitle.set_markup(&stadiaData.name);
	newFixed.add(&newTitle);

	let newDesc = Label::new(None);
	newDesc.set_markup(&stadiaData.desc);
	newFixed.add(&newDesc);

	

	window.add(&newFixed);
	window.show_all();

	return "halt".to_string();
}
