use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let couleur_cpu = "#b794f4";
    let path = "/proc/stat";

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(Box::new(e)),
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut prev_stats: (f64, f64, f64) = (0.0, 0.0, 0.0);

    loop {
        if let Err(_) = reader.seek(SeekFrom::Start(0)) {
            break;
        }

        line.clear();

        if let Ok(_) = reader.read_line(&mut line) {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() > 4 {
                let user = match parts[1].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => 0.0,
                };
                
                let system = match parts[3].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => 0.0,
                };
                
                let idle = match parts[4].parse::<f64>() {
                    Ok(n) => n,
                    Err(_) => 0.0,
                };

                let delta_user = user - prev_stats.0;
                let delta_system = system - prev_stats.1;
                let delta_idle = idle - prev_stats.2;

                let total_delta = delta_user + delta_system + delta_idle;

                let usage = if total_delta > 0.0 {
                    ((delta_user + delta_system) * 100.0) / total_delta
                } else {
                    0.0
                };

                println!("<span foreground='{}'>CPU {:.0}%</span>", couleur_cpu, usage);

                prev_stats = (user, system, idle);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}