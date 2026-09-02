use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;
use waybar::lock::{LockImageGenerator, ScreenLocker, SwaylockLocker};

pub fn lock_now() {
    let generator = LockImageGenerator::default();
    let image_to_lock = generator.get_or_render_recent(Duration::from_secs(180));
    let locker = SwaylockLocker::default();
    let _ = locker.lock(&image_to_lock);
}

pub fn run_daemon() {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let pid_file = format!("{}/lock_edt_daemon.pid", runtime_dir);

    if let Ok(old_pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(old_pid) = old_pid_str.trim().parse::<i32>() {
            let current_pid = std::process::id() as i32;
            if old_pid != current_pid {
                let _ = Command::new("kill").arg("-9").arg(old_pid.to_string()).status();
            }
        }
    }
    let _ = fs::write(&pid_file, std::process::id().to_string());

    println!(
        "[lock_edt] Démarrage du daemon de pré-génération du lockscreen (PID {})...",
        std::process::id()
    );

    let generator = LockImageGenerator::default();
    generator.render();

    loop {
        thread::sleep(Duration::from_secs(30));
        generator.render();
    }
}

fn print_help() {
    println!("Usage: lock_edt [OPTIONS]");
    println!();
    println!("Options:");
    println!("  (aucun)       Verrouille l'écran instantanément avec swaylock");
    println!("  --daemon, -d  Exécute le service en arrière-plan (rafraîchit le lockscreen)");
    println!("  --generate, -g Génère l'image composite une fois et quitte");
    println!("  --help, -h    Affiche cette aide");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("--daemon" | "daemon" | "-d") => run_daemon(),
        Some("--generate" | "generate" | "-g") => {
            let generator = LockImageGenerator::default();
            let img = generator.render();
            println!("Image générée : {}", img);
        }
        Some("--help" | "-h") => print_help(),
        _ => lock_now(),
    }
}
