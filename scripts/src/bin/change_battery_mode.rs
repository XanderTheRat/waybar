use std::env;
use std::fs;
use std::path::Path;

fn main() ->  Result<(), Box<dyn std::error::Error>> {
	let home = env::var("HOME").expect("$HOME not found");
	
	let state_file = format!("{}/.config/waybar/scripts/battery_state", home);

	let mut mode = if Path::new(&state_file).exists() {
	        fs::read_to_string(&state_file)?.trim().parse::<u8>()?
	} else {
		println!("File not found : {}", state_file);
		1
	};

	mode = if mode == 1 { 2 } else { 1 };
	fs::write(state_file, mode.to_string())?;

	Ok(())
}
