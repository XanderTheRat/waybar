use std::fs;
use std::path::Path;

fn main() ->  Result<(), Box<dyn std::error::Error>> {
	let state_file = "/home/martin/.config/scripts.rs/waybar/battery_state";
	let mut mode = if Path::new(state_file).exists() {
	        fs::read_to_string(state_file)?.trim().parse::<u8>().unwrap_or(1)
	} else {
		println!("Erreur de nom de fichier");
		1
	};

	mode = if mode == 1 { 2 } else { 1 };
	fs::write(state_file, mode.to_string())?;

	Ok(())
}

