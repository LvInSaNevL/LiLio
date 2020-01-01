extern crate gtk;
use crate::market;
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

// pub fn menuManager(app: App, res: Vec<i32>) {
//     let _menu = true;

//     loop {
//         if _menu { _menu = mainMenu(app, res); }
//         else { _menu = marketMenu(app, res); }
//     }
// }

pub fn mainMenu(app: App, res: Vec<i32>) {
    let buttonSize = (res[1] / 3);
    let appData = market::ReadMarket(false);
    let menuBox = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    menuBox.set_spacing(buttonSize / 2);
    menuBox.set_margin_start(100);
    menuBox.set_margin_end(100);
    let scroll = gtk::ScrolledWindow::new(None, None);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

    // The market button always needs to be created so it goes in its own code block
    let marketButton = Button::new_with_label("Market");
    marketButton.set_size_request(buttonSize, buttonSize);
    marketButton.connect_clicked(move |_| {
        println!("Hello, World!");
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

    // Adds everything to the window
    scroll.add(&menuBox);
    app.window.add(&scroll);
    app.window.show_all();
}
