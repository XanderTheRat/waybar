use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let colors = waybar::Colors::load();
	let color_battery = colors.get("color_battery");
	let color_charging = colors.get("color_charging");
	let color_plugged = colors.get("color_plugged");
	let color_low_battery = colors.get("color_low_battery");
	let color_full = colors.get("color_full");
	let color_critical = colors.get("color_critical");

	let state = ["", "", "", "", ""];
    let show_battery = || -> Result<(usize, String, String), Box<dyn std::error::Error>>{
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
			color = color_charging.clone();		
		}
		else if battery_state == "Not charging" {
			color = color_plugged.clone();
		}
		else if battery_state == "Full" {
			color = color_full.clone();
		}
		else if battery_percent > warning_state {
			color = color_battery.clone();
		}
		else if battery_percent > critical_state {
			color = color_low_battery.clone();
		}
		else {
			color = color_critical.clone();
		}
		Ok((battery_percent, battery_state, color))							
	};

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
						println!("<span foreground='{}'>Battery full</span>", color_full);					
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
						color = &color_charging;
						println!("<span foreground='{}'>Batterie charging</span>", color);
					}else if status == "Full" {
						color = &color_full;
						println!("<span foreground='{}'>Battery full</span>", color);					
					}
					else {
						color = &color_battery;
						let Ok((remaining_hours, remaining_minute)) = show_remaining_time() else { todo!() };
						println!("<span foreground='{}'>{} hours {} remaining</span>", color,remaining_hours, remaining_minute)
					}
				} 
				_ => {}
			}
		}
		else {
			println!("<span foreground='{}'>Batterie sur secteur</span>", color_full);
		}
	} else {
		println!("Fichier non trouvé : {}", state_file);
	};
	Ok(())
}
