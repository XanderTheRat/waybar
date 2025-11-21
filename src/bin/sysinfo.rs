use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let couleur_cpu = "#b794f4";
	let couleur_mem = "#f687b3";

	let output_cpu = Command::new("sh").arg("-c").arg("awk -v RS=\"\" '{getline; cpu=($2+$4)*100/($2+$4+$5); printf \"%d\", cpu}' /proc/stat").output().expect("Erreur");
	let cpu = String::from_utf8_lossy(&output_cpu.stdout).trim().to_string();
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
				println!("<span foreground='{}'>CPU {}%</span> | <span foreground='{}'>MEM {}%</span>", couleur_cpu, cpu, couleur_mem, percent);

			}
		}
	}
	Ok(())
}
