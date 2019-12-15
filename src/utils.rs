use std::process::{Command, Stdio};

pub fn splashPrint() {
    let splash_array = [
    "  | |      (_)    | |      (_)     ___",   
    "  | |__    | |    | |__    | |    / _ \\",  
    "  |____|  _|_|_   |____|  _|_|_   \\___/ ", 
    "_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|_|\"\"\"\"\"|", 
    "\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-'\"`-0-0-' "
    ];

    for line in splash_array.iter() {
        println!("{}", line)
    }
}

// Gets the screen dimensions
// xrandr | grep -w connected  | awk -F'[ +]' '{print $3}'
pub fn getResolution() -> Vec<i32> {
    let resOut = Command::new("xrandr")
                    .output()
                    .unwrap();
    let rawRes = String::from_utf8(resOut.stdout).unwrap();
    let displayIndex = rawRes.find("current ").unwrap() + 8;

    let formattedString: Vec<&str> = rawRes[displayIndex..(displayIndex+11)].split(" x ").collect();
    let mut formattedInt = Vec::<i32>::new();

    formattedInt.push(formattedString[0].parse().unwrap());
    formattedInt.push(formattedString[1].parse().unwrap());

    return formattedInt;
}