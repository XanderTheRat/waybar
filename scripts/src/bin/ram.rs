use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let couleur_mem = "#f687b3";
    let path = "/proc/meminfo";

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(Box::new(e)),
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        if let Err(_) = reader.seek(SeekFrom::Start(0)) {
            break;
        }

        let mut mem_total: f64 = 0.0;
        let mut mem_available: f64 = 0.0;

        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break, 
                Ok(_) => {
                    if line.starts_with("MemTotal:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(val) = parts[1].parse::<f64>() {
                                mem_total = val;
                            }
                        }
                    } else if line.starts_with("MemAvailable:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(val) = parts[1].parse::<f64>() {
                                mem_available = val;
                            }
                        }
                    }

                    if mem_total > 0.0 && mem_available > 0.0 {
                        break;
                    }
                }
                Err(_) => break,
            }
        }

        if mem_total > 0.0 {
            let used = mem_total - mem_available;
            let percent = (used * 100.0 / mem_total).round() as u32;
            
            println!("<span foreground='{}'>MEM {}%</span>", couleur_mem, percent);
        }

        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}