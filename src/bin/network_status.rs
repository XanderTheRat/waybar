use std::fs;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let modefile = "/home/martin/.config/scripts.rs/waybar_scripts/network_status";
	let file_exist = fs::exists(modefile);
	let exists = match file_exist {
		Ok(value) => value,
		Err(_) => false,
	};
	
	let network_mode = fs::read_to_string(modefile)?.trim().parse::<u8>()?;
	let _: Result<f64, Box<dyn std::error::Error>> = Ok(network_mode.into());
	
	if exists {
		match network_mode {
			1_u8 => {
				let nmcli_output = Command::new("nmcli").arg("-t").arg("-f").arg("TYPE,STATE,CONNECTION").arg("dev").output()?;
				let nmcli = String::from_utf8_lossy(&nmcli_output.stdout).trim().to_string();
				let ssid : Vec<&str> =nmcli.split_terminator(&[':','\n']).collect() ;

				if ssid[2].is_empty() {
					println!("<span foreground='#f87171'>⚠ Disconnected</span>");
				}
				else {
					println!("<span foreground='#63b3ed'> {}</span>", ssid[2]);
				}

			},
			2_u8 | 3_u8 => {
				let ip_output = Command::new("ip").arg("-br").arg("a").output()?;
				let ip = String::from_utf8_lossy(&ip_output.stdout).trim().to_string();

				// You have to adapt the chain that is used to your network interfaces. If the active interface is first when you execute ip -br a, replace 'rsplit' with 'split'
				let vec_ip: Vec<&str> = ip.rsplit('\n').collect();
				let output_ip: Vec<&str> = vec_ip[0].split_whitespace().collect();

				let color:&str;
				let status_mode: i8;
				if network_mode == 2 {
					color = "#a78bfa";
					status_mode = 4;
				}
				else {
					color = "#fb923c";
					status_mode = 6;
				}

				println!("<span foreground='{}'>ipv{}:</span><span foreground='#63b3ed'> {}</span>",color, status_mode, output_ip[network_mode as usize]);
			}
			_ => todo!()
		}
	}
	else {
		println!("Fichier non trouvé : {}", modefile);
	}
	Ok(())
}
