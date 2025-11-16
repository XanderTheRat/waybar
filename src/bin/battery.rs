use std::process::Command;
use std::fs;
use std::path::Path;
use clap::Parser;

// TODO : Rajouter les options de paramètres
// TODO : Faire en sorte que : 
//	- 1 -> Afficher la batterie
//	- 2 -> Afficher le temps de batterie/charge restant

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let args = Args::parse();
	let etat = ["", "", "", "", ""];

    fn affiche_batterie() -> (usize, String, &'static str) {
		let couleur_batterie_bon = "#68d391";
		let couleur_batterie_chargement = "#48bb78";
		let couleur_batterie_branche = "#38b2ac";
		let couleur_batterie_faible = "#f6ad55";
								
		let output_batterie = Command::new("cat").arg("/sys/class/power_supply/BAT0/capacity").output().expect("Erreur");
		let output_batterie_etat = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output().expect("Erreur");
		let batterie_etat_pas_trim = String::from_utf8_lossy(&output_batterie_etat.stdout);
									
		let batterie_output = String::from_utf8_lossy(&output_batterie.stdout);
		let batterie = batterie_output.trim();
		let batterie_etat = batterie_etat_pas_trim.trim().to_string();
		let batterie_pourcent = batterie.parse::<usize>().unwrap();
		let couleur;

		if batterie_etat == "Charging" {
			couleur = couleur_batterie_chargement;		
		}
		else if batterie_etat == "Not Charging" {
			couleur = couleur_batterie_branche;
		}
		else if batterie_pourcent > 30 {
				couleur = couleur_batterie_bon;
		}
		else if batterie_pourcent > 15 {
			couleur = couleur_batterie_faible;
		}
		else {
			couleur = "#FFFFFF"
		}
		(batterie_pourcent, batterie_etat, couleur)							
	}

	fn affichage_tps_restant() -> (i32, i32){
		let energy_now = Command::new("cat").arg("/sys/class/power_supply/BAT0/charge_now").output().expect("Erreur");
		let energy_full = Command::new("cat").arg("/sys/class/power_supply/BAT0/charge_full").output().expect("Erreur");
		let power_now = Command::new("cat").arg("/sys/class/power_supply/BAT0/current_now").output().expect("Erreur");    
		let output_batterie_etat = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output().expect("Erreur");
			     		
		let energie_restante_pas_trim = String::from_utf8_lossy(&energy_now.stdout);
		let capacite_total_batterie_pas_trim = String::from_utf8_lossy(&energy_full.stdout);
		let energie_consommee_pas_trim = String::from_utf8_lossy(&power_now.stdout);
		let batterie_etat_pas_trim = String::from_utf8_lossy(&output_batterie_etat.stdout); 				
				
		let energie_consommee = energie_consommee_pas_trim.trim().parse::<f64>().unwrap();
		let capacite_total_batterie = capacite_total_batterie_pas_trim.trim().parse::<f64>().unwrap();
		let energie_restante = energie_restante_pas_trim.trim().parse::<f64>().unwrap();
		let batterie_etat = batterie_etat_pas_trim.trim();

	    let mut heures_restantes;
		if batterie_etat == "Charging" || batterie_etat == "Not Charging" {
			heures_restantes = (capacite_total_batterie - energie_restante)/energie_consommee; 
		} 
		else {
			heures_restantes = energie_restante/energie_consommee;
		}

		let minutes_depuis_heures = heures_restantes * 60.0;
		let mut minutes_restantes = minutes_depuis_heures.round() as i32;
		minutes_restantes /= 60;
		(heures_restantes.round() as i32, minutes_restantes)   		    		
	}

	// ----------------------------------------------------------------------
	let state_file = "/home/martin/.config/scripts.rs/waybar/battery_state";
	let mode;

	if Path::new(state_file).exists() {
	        mode = fs::read_to_string(state_file)?.trim().parse::<u8>().unwrap_or(1);
			match mode {
				1 => {
				let (batterie_pourcent, batterie_etat, couleur) = affiche_batterie();
				if batterie_etat == "Charging" {
					println!("<span foreground='{}'>{}% </span>",couleur, batterie_pourcent);		
				}
				else if batterie_etat == "Not Charging" {
					println!("<span foreground='{}'>{}% </span>",  couleur, batterie_pourcent)
				}
				else if batterie_pourcent > 30 {
					println!("<span foreground='{}'>{}% {}</span>", couleur, batterie_pourcent, etat[batterie_pourcent/20]);
				}
				else {
						println!("<span foreground={}>{}% {}</span>", couleur, batterie_pourcent, etat[batterie_pourcent/20])
					}	
				}
				2 => {
					let couleur = "#68d391";
					let (heures_restantes, minutes_restantes) = affichage_tps_restant();
					println!("<span foreground='{}'>{} heure(s) {} minutes</span>", couleur,heures_restantes, minutes_restantes)
				} 
				_ => {}
		}
	} else {
		println!("Erreur de nom de fichier");
	};

	Ok(())
}				

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
	#[arg(long, action = clap::ArgAction::SetTrue)]
    toggle : bool
}
