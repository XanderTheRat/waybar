use std::env;
use std::path::Path;
use std::fs;

// Error au changement de mode, à régler

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let home = env::var("HOME").expect("$HOME not found");

	let state_file = format!("{}/.config/waybar/scripts/network_status", home);
	let mode_read = if Path::new(&state_file).exists() {
	        fs::read_to_string(&state_file)?.trim().parse::<u8>()?
	} else {
		println!("File not found : {}", state_file);
		1
	};

	let mode_write:u8;

	match mode_read {
		1_u8 => mode_write = 2,
		2_u8 => mode_write = 3,
		_ => mode_write = 1
	}

	fs::write(state_file, mode_write.to_string())?;

	Ok(())
}
