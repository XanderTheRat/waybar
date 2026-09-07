use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;

fn save_battery_color_binary(color_hex: &str) {
	let hex = color_hex.trim_start_matches('#');
	if hex.len() != 6 {
		return;
	}
	if let (Ok(r), Ok(g), Ok(b)) = (
		u8::from_str_radix(&hex[0..2], 16),
		u8::from_str_radix(&hex[2..4], 16),
		u8::from_str_radix(&hex[4..6], 16),
	) {
		let bytes = [r, g, b];
		if let Ok(home) = env::var("HOME") {
			let path = format!("{}/.config/waybar/scripts/battery_color.bin", home);
			let _ = fs::write(path, bytes);
		}
		if let Ok(runtime_dir) = env::var("XDG_RUNTIME_DIR") {
			let path = format!("{}/battery_color.bin", runtime_dir);
			let _ = fs::write(path, bytes);
		}
	}
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let state = ["", "", "", "", ""];
    fn show_battery() -> Result<(usize, String, &'static str), Box<dyn std::error::Error>>{
		let color_battery = "#68d391";
		let color_charging = "#48bb78";
		let color_plugged = "#38b2ac";
		let color_low_battery = "#f6ad55";
		let color_full = "#40B792";

		let warning_state = 30;
		let critical_state = 15;

		let output_battery = Command::new("cat").arg("/sys/class/power_supply/BAT0/capacity").output()?;
		let output_battery_state = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output()?;
		let battery_state_not_trim = String::from_utf8_lossy(&output_battery_state.stdout);
		
		let battery_output = String::from_utf8_lossy(&output_battery.stdout);
		let battery = battery_output.trim();
		let battery_state = battery_state_not_trim.trim().to_string();
		let battery_percent = battery.parse::<usize>()?;
		let _: Result<f64, Box<dyn std::error::Error>> = Ok(battery_percent as f64);
		let color;

		if battery_state == "Charging" {
			color = color_charging;		
		}
		else if battery_state == "Not charging" {
			color = color_plugged;
		}
		else if battery_state == "Full" {
			color = color_full;
		}
		else if battery_percent > warning_state {
				color = color_battery;
		}
		else if battery_percent > critical_state {
			color = color_low_battery;
		}
		else {
			color = "#FFFFFF";
		}
		save_battery_color_binary(color);
		Ok((battery_percent, battery_state, color))							
	}

	fn show_remaining_time() -> Result<(i32, i32), Box<dyn std::error::Error>>{
		let energy_now = Command::new("cat").arg("/sys/class/power_supply/BAT0/charge_now").output()?;
		let energy_full = Command::new("cat").arg("/sys/class/power_supply/BAT0/charge_full").output()?;
		let power_now = Command::new("cat").arg("/sys/class/power_supply/BAT0/current_now").output()?;    
		let output_battery_state = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output()?;
  		
		let remaining_power_not_trim = String::from_utf8_lossy(&energy_now.stdout);
		let total_battery_capacity_not_trim = String::from_utf8_lossy(&energy_full.stdout);
		let used_battery_not_trim = String::from_utf8_lossy(&power_now.stdout);
		let battery_state_not_trim = String::from_utf8_lossy(&output_battery_state.stdout); 				
		
		let used_battery = used_battery_not_trim.trim().parse::<f64>()?;
		let total_battery_capacity = total_battery_capacity_not_trim.trim().parse::<f64>()?;
		let remaining_power = remaining_power_not_trim.trim().parse::<f64>()?;
		let battery_state = battery_state_not_trim.trim();

		let _ = Ok::<f64, Box<dyn std::error::Error>>(used_battery);
		let _ = Ok::<f64, Box<dyn std::error::Error>>(total_battery_capacity);
		let _ = Ok::<f64, Box<dyn std::error::Error>>(remaining_power);

	    let remaining_hours:f64;
	    // TODO : fix the output of time
	    // Using calc of battery_state(n-1) - battery_state(n) for aproximating time

		if battery_state == "Charging" || battery_state == "Not Charging" {
        	remaining_hours=(total_battery_capacity - remaining_power) / used_battery;
    	} else {
	       	remaining_hours= remaining_power / used_battery;
    	}

		let minute = remaining_hours * 60.0;
		let mut remaining_minute = minute.round() as i32;
		remaining_minute /= 60;

		Ok((remaining_hours.round() as i32, remaining_minute))   		    		
	}

	let home = env::var("HOME").expect("$HOME not found");
	
	let state_file = format!("{}/.config/waybar/scripts/battery_state", home);
	let mode;

	if Path::new(&state_file).exists() {
	        mode = fs::read_to_string(state_file)?.trim().parse::<u8>()?;
			let _: Result<f64, Box<dyn std::error::Error>> = Ok(mode.into());
			if Path::new("/sys/class/power_supply/BAT0").exists() {
				match mode {
				1 => {
					let Ok((battery_percent, battery_state, color)) = show_battery() else { todo!() };
					if battery_state == "Charging" {
						println!("<span foreground='{}'>{}% </span>",color, battery_percent);		
					}
					else if battery_state == "Not charging" {
						println!("<span foreground='{}'>{}% </span>",  color, battery_percent)
					}
					else if battery_state == "Full" {
						let color = "#40B792";
						println!("<span foreground='{}'>Battery full</span>", color);					
					}
					else {
						println!("<span foreground='{}'>{}% {}</span>", color, battery_percent, state[(battery_percent/20).min(4)]);
					}	
				}
				2 => {
					let _ = show_battery();
					let battery_state = Command::new("cat").arg("/sys/class/power_supply/BAT0/status").output()?;
					let battery_state_not_trim = String::from_utf8_lossy(&battery_state.stdout);
					let status = battery_state_not_trim.trim().to_string();

					let color:&str;
					if status == "Charging" || status == "Not charging" {
						color = "#40B792";
						println!("<span foreground='{}'>Batterie charging</span>", color);
					}else if status == "Full" {
						color = "#40B792";
						println!("<span foreground='{}'>Battery full</span>", color);					
					}
					else {
						color = "#68d391";
						let Ok((remaining_hours, remaining_minute)) = show_remaining_time() else { todo!() };
						println!("<span foreground='{}'>{} hours {} remaining</span>", color,remaining_hours, remaining_minute)
					}
				} 
				_ => {}
			}
		}
		else {
			let color = "#40B792";
			save_battery_color_binary(color);
			println!("<span foreground='{}'>Batterie sur secteur</span>", color);
		}
	} else {
		println!("Fichier non trouvé : {}", state_file);
	};
	Ok(())
}				
