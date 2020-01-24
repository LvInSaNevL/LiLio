extern crate gdk;
extern crate gtk;
extern crate hex;

pub fn stylizer(colorA: String, colorB: String) {
	// The base stylesheet for LiLio
	let rawStyle = "
        button {
            /* If we don't put it, the yellow background won't be visible */
            background-image: none;
        }
        window {
            background-image: url('./noise.png'), linear-gradient(130deg, #[colorA], #[colorB]);
            color: blue;
            font-weight: bold;
        }";
	// Creates the actual stylesheet
	let mut formattedStyle = str::replace(rawStyle, "[colorA]", &colorA);
	formattedStyle = str::replace(&formattedStyle, "[colorB]", &colorB);

	// Creates the CSS Provider and adds the styling
	let provider = gtk::CssProvider::new();
	gtk::CssProviderExt::load_from_data(&provider, formattedStyle.as_bytes())
		.expect("Failed to add CSS to the program");

	// We give the CssProvided to the default screen so the CSS rules we added
	// can be applied to our window.
	gtk::StyleContext::add_provider_for_screen(
		&gdk::Screen::get_default().expect("Error initializing gtk css provider."),
		&provider,
		gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
}
