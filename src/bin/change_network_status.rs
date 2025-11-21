use std::path::Path;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let state_file = "/home/martin/.config/scripts.rs/waybar_scripts/network_status";
	let mode_read = if Path::new(state_file).exists() {
	        fs::read_to_string(state_file)?.trim().parse::<u8>()?
	} else {
		println!("Fichier non trouvé : {}", state_file);
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
