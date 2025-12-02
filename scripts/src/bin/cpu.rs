use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let couleur_cpu = "#b794f4";

	let output_cpu = Command::new("sh").arg("-c").arg("awk -v RS=\"\" '{getline; cpu=($2+$4)*100/($2+$4+$5); printf \"%d\", cpu}' /proc/stat").output().expect("Erreur");
	let cpu = String::from_utf8_lossy(&output_cpu.stdout).trim().to_string();

	println!("<span foreground='{}'>CPU {}%</span>", couleur_cpu, cpu);	
	Ok(())
}
