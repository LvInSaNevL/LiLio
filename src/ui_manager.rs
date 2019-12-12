extern crate gtk;
use gtk::*;

pub struct App {
    pub window: Window,
    pub header: Header,
}

pub struct Header {
    pub container: HeaderBar
}

// Creating the actual root window for LiLio
impl App {
    pub fn new() -> App {
        // Defining the root window the header bar
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();

        // Setting the window name for the user and wm
        window.set_titlebar(&header.container);
        window.set_title("LiLio");
        window.set_wmclass("lilio", "LiLio");
        window.set_default_size(800, 480);

        // Exit button logic
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return to the main program
        App { window, header }
    }
}

// Creates a proper header with window controls
impl Header {
    fn new() -> Header {
        // Creates the header container
        let container = HeaderBar::new();

        // Sets the header text
        container.set_title("LiLio");
        container.set_show_close_button(true);

        // Jumps back to main logic
        Header { container }
    }
}
