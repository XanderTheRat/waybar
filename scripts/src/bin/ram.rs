use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let couleur_mem = "#f687b3";

	let output_mem = Command::new("free").arg("-m").stdout(Stdio::piped()).spawn().unwrap().stdout.expect("Erreur");

	let reader = BufReader::new(output_mem);

	for instance in reader.lines() {
		let line:String = instance?;
		if line.starts_with("Mem") {
			let memory:Vec<&str> = line.split_whitespace().collect();
			if memory.len() >= 3 {
				let used:f64 = memory[2].parse()?;
				let total: f64 = memory[1].parse()?;

				let percent = (used*100.0/total).round() as u32;
				println!("<span foreground='{}'>MEM {}%</span>", couleur_mem, percent);

			}
		}
	}
	Ok(())
}
