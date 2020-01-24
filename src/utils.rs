use std::process::Command;

pub fn splashPrint() {
	let splash_array = [
		"  | |      (_)    | |      (_)     ___",
		"  | |__    | |    | |__    | |    / _ \\",
		"  |____|  _|_|_   |____|  _|_|_   \\___/ ",
		"_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|",
		"\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-' ",
	];

	for line in splash_array.iter() {
		println!("{}", line)
	}
}

// Gets the screen dimensions
// xrandr | grep -w connected  | awk -F'[ +]' '{print $3}'
pub fn getResolution() -> Vec<i32> {
	let resOut = Command::new("xrandr").output().unwrap();
	let rawRes = String::from_utf8(resOut.stdout).unwrap();
	let displayIndex = rawRes.find("current ").unwrap() + 8;

	let formattedString: Vec<&str> = rawRes[displayIndex..(displayIndex + 11)]
		.split(" x ")
		.collect();
	let mut formattedInt = Vec::<i32>::new();

	formattedInt.push(formattedString[0].parse().unwrap());
	formattedInt.push(formattedString[1].parse().unwrap());

	return formattedInt;
}

pub fn openWindow(target: String) {
	let arguments = [
		"export GOOGLE_API_KEY=\"AIzaSyDbjeCeu0_TLxrdk41e_m1Ds3I99r01bng\"",
		"export GOOGLE_DEFAULT_CLIENT_ID=\"1052749106742-o2mvdjad6s9hfu6r38fjdc0koupluarf.apps.googleusercontent.com\"",
		"export GOOGLE_DEFAULT_CLIENT_SECRET=\"TTjdPWjrZ1iAeT632IV9DiEB\""
	];

	for command in arguments.iter() {
		Command::new(command).spawn();
	}

	Command::new("sh")
		.arg("-c")
		.arg(format!(
			"~/.LiLio/chromium/chrome-linux/chrome --disable-notification {}",
			target
		))
		.spawn()
		.expect("Failed to launch Chromium");
}
