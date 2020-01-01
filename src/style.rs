extern crate gdk;
extern crate gtk;
use gtk::prelude::*;

pub fn stylizer(color: String) {
    // The base stylesheet for LiLio
    let rawStyle = "
        button {
            /* If we don't put it, the yellow background won't be visible */
            background-image: none;
        }
        window {
            background-image: -gtk-gradient (linear,
                                            0 0, 1 0,
                                            color-stop(0, #[colorA]),
                                            color-stop(1, #[colorB]));
            color: blue;
            font-weight: bold;
        }";
    // Creates the actual stylesheet
    let mut formattedStyle = str::replace(rawStyle, "[colorA]", &color);
            formattedStyle = str::replace(&formattedStyle, "[colorB]", &color);

    println!("{}", formattedStyle);

    // Creates the CSS Provider and adds the styling
    let provider = gtk::CssProvider::new();
    gtk::CssProviderExt::load_from_data(&provider, formattedStyle.as_bytes());

    // We give the CssProvided to the default screen so the CSS rules we added
    // can be applied to our window.
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
