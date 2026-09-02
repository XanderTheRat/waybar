use chrono::{Local, Utc};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use waybar::edt::Edt;

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn get_wallpaper_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/martin".to_string());
    let default_wp = PathBuf::from(&home).join("wallpaper/outerwilds2.jpg");
    if default_wp.exists() {
        return default_wp;
    }
    let alt_wp = PathBuf::from(&home).join("wallpaper/outerwilds1.jpg");
    if alt_wp.exists() {
        return alt_wp;
    }
    let alt_wp2 = PathBuf::from(&home).join("wallpaper/outerwilds3.jpg");
    if alt_wp2.exists() {
        return alt_wp2;
    }
    default_wp
}

struct ColorTheme {
    card_bg_start: &'static str,
    card_bg_end: &'static str,
    card_border: &'static str,
    badge_bg: &'static str,
    badge_border: &'static str,
    badge_color: &'static str,
}

impl ColorTheme {
    fn for_course_type(course_type: &str) -> Self {
        let ct_upper = course_type.trim().to_uppercase();
        // couleurs cours
        if ct_upper.contains("CM") {
            Self {
                card_bg_start: "#231e0f",
                card_bg_end: "#362c12",
                card_border: "rgba(251, 191, 36, 0.45)",
                badge_bg: "rgba(245, 158, 11, 0.22)",
                badge_border: "rgba(245, 158, 11, 0.55)",
                badge_color: "#FBBF24",
            }
        } else if ct_upper.contains("TD") {
            Self {
                card_bg_start: "#24101e",
                card_bg_end: "#38162f",
                card_border: "rgba(244, 114, 182, 0.45)",
                badge_bg: "rgba(236, 72, 153, 0.22)",
                badge_border: "rgba(236, 72, 153, 0.55)",
                badge_color: "#F472B6",
            }
        } else if ct_upper.contains("TP") {
            Self {
                card_bg_start: "#0d1b2a",
                card_bg_end: "#152c42",
                card_border: "rgba(56, 189, 248, 0.45)",
                badge_bg: "rgba(14, 165, 233, 0.22)",
                badge_border: "rgba(14, 165, 233, 0.55)",
                badge_color: "#38BDF8",
            }
        } else {
            Self {
                card_bg_start: "#0d2318",
                card_bg_end: "#143725",
                card_border: "rgba(74, 222, 128, 0.45)",
                badge_bg: "rgba(16, 185, 129, 0.22)",
                badge_border: "rgba(16, 185, 129, 0.55)",
                badge_color: "#4ADE80",
            }
        }
    }
}

fn generate_svg(
    badge: &str,
    theme: &ColorTheme,
    title1: &str,
    title2: &str,
    group_info: &str,
) -> String {
    let esc_badge = escape_xml(badge);
    let esc_title1 = escape_xml(title1);
    let esc_title2 = escape_xml(title2);
    let esc_group = escape_xml(group_info);

    let title1_len = title1.chars().count();
    let title1_font_size = if title1_len > 38 {
        18
    } else if title1_len > 26 {
        21
    } else {
        24
    };

    // position widget edt
    let card_x = 90;
    let card_y = 740;
    let card_width = 580;
    let card_height = 210;
    let card_cx = card_x + card_width / 2;

    let badge_width = (esc_badge.chars().count() * 8 + 36).max(120);
    let badge_x = card_cx - (badge_width as i32 / 2);
    let badge_y = card_y + 20;

    let sep_x1 = card_x + 35;
    let sep_x2 = card_x + card_width - 35;
    let sep_y = card_y + 148;

    // position widget heure
    let current_time = Local::now().format("%H:%M").to_string();
    let clock_x = 1275;
    let clock_y = 445;

    format!(
        r###"<svg width="1920" height="1080" viewBox="0 0 1920 1080" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="edtCardGrad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="{card_bg_start}" stop-opacity="0.75"/>
      <stop offset="100%" stop-color="{card_bg_end}" stop-opacity="0.65"/>
    </linearGradient>
    <filter id="cardShadow" x="-10%" y="-10%" width="120%" height="130%">
      <feDropShadow dx="0" dy="10" stdDeviation="18" flood-color="#000000" flood-opacity="0.6"/>
    </filter>
    <filter id="textGlow" x="-20%" y="-20%" width="140%" height="140%">
      <feDropShadow dx="0" dy="3" stdDeviation="6" flood-color="#000000" flood-opacity="0.75"/>
    </filter>
  </defs>

  <!-- heure -->
  <text filter="url(#textGlow)" x="{clock_x}" y="{clock_y}" font-family="Noto Sans, sans-serif" font-size="58" font-weight="700" fill="#FFFFFF" text-anchor="middle" letter-spacing="2">
    {current_time}
  </text>

  <!-- edt -->
  <g filter="url(#cardShadow)">
    <rect x="{card_x}" y="{card_y}" width="{card_width}" height="{card_height}" rx="20" ry="20"
          fill="url(#edtCardGrad)"
          stroke="{card_border}"
          stroke-width="1.5" />
  </g>

  <rect x="{badge_x}" y="{badge_y}" width="{badge_width}" height="24" rx="12" ry="12"
        fill="{badge_bg}"
        stroke="{badge_border}"
        stroke-width="1" />
  <text x="{card_cx}" y="{badge_text_y}" font-family="Noto Sans, sans-serif" font-size="11" font-weight="700" fill="{badge_color}" text-anchor="middle" letter-spacing="1.5">
    {esc_badge}
  </text>

  <!-- prochn crs -->
  <text x="{card_cx}" y="{title1_y}" font-family="Noto Sans, sans-serif" font-size="{title1_font_size}" font-weight="700" fill="#FFFFFF" text-anchor="middle">
    {esc_title1}
  </text>

  <!-- horaires -->
  <text x="{card_cx}" y="{title2_y}" font-family="Noto Sans, sans-serif" font-size="16" font-weight="500" fill="#E2E8F0" text-anchor="middle">
    {esc_title2}
  </text>

  <line x1="{sep_x1}" y1="{sep_y}" x2="{sep_x2}" y2="{sep_y}" stroke="rgba(255, 255, 255, 0.12)" stroke-width="1" />

  <text x="{card_cx}" y="{group_y}" font-family="Noto Sans, sans-serif" font-size="13" font-weight="400" fill="#94A3B8" text-anchor="middle" letter-spacing="0.5">
    {esc_group}
  </text>
</svg>"###,
        card_bg_start = theme.card_bg_start,
        card_bg_end = theme.card_bg_end,
        card_border = theme.card_border,
        badge_bg = theme.badge_bg,
        badge_border = theme.badge_border,
        badge_color = theme.badge_color,
        card_x = card_x,
        card_y = card_y,
        card_width = card_width,
        card_height = card_height,
        card_cx = card_cx,
        badge_x = badge_x,
        badge_y = badge_y,
        badge_width = badge_width,
        badge_text_y = badge_y + 16,
        title1_font_size = title1_font_size,
        title1_y = card_y + 78,
        title2_y = card_y + 118,
        sep_x1 = sep_x1,
        sep_x2 = sep_x2,
        sep_y = sep_y,
        group_y = card_y + 180,
        clock_x = clock_x,
        clock_y = clock_y,
        current_time = current_time,
        esc_badge = esc_badge,
        esc_title1 = esc_title1,
        esc_title2 = esc_title2,
        esc_group = esc_group,
    )
}

fn get_lock_paths() -> (String, String, String) {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let svg_path = format!("{}/swaylock_edt.svg", runtime_dir);
    let tmp_output_path = format!("{}/swaylock_lock.tmp.png", runtime_dir);
    let final_output_path = format!("{}/swaylock_lock.png", runtime_dir);
    (svg_path, tmp_output_path, final_output_path)
}

pub fn render_lock_screen() -> String {
    let wallpaper = get_wallpaper_path();
    let (svg_path, tmp_output_path, final_output_path) = get_lock_paths();

    let edt = Edt::default();
    let schedule_res = edt.fetch_or_cached(Duration::from_secs(300));

    let (badge, theme, title1, title2, group_info) = match schedule_res {
        Ok(schedule) => {
            let base_group = format!("Groupe {}{} • {}", schedule.group, schedule.tp, schedule.year);
            if let Some(course) = schedule.active_or_next_course() {
                let now_utc = Utc::now();
                let start_utc = course.start_utc().unwrap_or(now_utc);
                let is_upcoming = start_utc > now_utc;

                let badge = if is_upcoming {
                    format!("PROCHAIN COURS ({})", course.course_type)
                } else {
                    format!("COURS EN COURS ({})", course.course_type)
                };

                let theme = ColorTheme::for_course_type(&course.course_type);
                let title1 = course.title.clone();

                let start_str = course
                    .start_local()
                    .map(|d| d.format("%H:%M").to_string())
                    .unwrap_or_else(|_| "--:--".to_string());
                let end_str = course
                    .end_local()
                    .map(|d| d.format("%H:%M").to_string())
                    .unwrap_or_else(|_| "--:--".to_string());
                let duration = course.duration_formatted();

                let mut title2 = format!("{} - {} ({})", start_str, end_str, duration);
                if !course.room.is_empty() {
                    title2.push_str(&format!("  •  Salle {}", course.room));
                }

                let mut group_str = base_group;
                if !course.teacher.is_empty() {
                    group_str.push_str(&format!("  •  {}", course.teacher));
                }

                (badge, theme, title1, title2, group_str)
            } else {
                let badge = "AUJOURD'HUI".to_string();
                let theme = ColorTheme::for_course_type("SAE");
                let title1 = "Aucun cours à venir".to_string();
                let title2 = "Fin des cours pour aujourd'hui".to_string();
                (badge, theme, title1, title2, base_group)
            }
        }
        Err(_) => {
            let badge = "EMPLOI DU TEMPS".to_string();
            let theme = ColorTheme::for_course_type("SAE");
            let title1 = "Emploi du temps indisponible".to_string();
            let title2 = "Planning hors ligne".to_string();
            let group_str = format!("Groupe {}{} • BUT3", Edt::DEFAULT_TD_GROUP, Edt::DEFAULT_TP_GROUP);
            (badge, theme, title1, title2, group_str)
        }
    };

    let svg_content = generate_svg(
        &badge,
        &theme,
        &title1,
        &title2,
        &group_info,
    );

    let previous_svg = fs::read_to_string(&svg_path).unwrap_or_default();
    if previous_svg == svg_content && Path::new(&final_output_path).exists() {
        return final_output_path;
    }

    let _ = fs::write(&svg_path, &svg_content);

    let composite_status = Command::new("magick")
        .arg(&wallpaper)
        .arg("(")
        .arg("-background")
        .arg("none")
        .arg(&svg_path)
        .arg(")")
        .arg("-composite")
        .arg(&tmp_output_path)
        .status();

    match composite_status {
        Ok(status) if status.success() && Path::new(&tmp_output_path).exists() => {
            let _ = fs::rename(&tmp_output_path, &final_output_path);
            final_output_path
        }
        _ => wallpaper.to_string_lossy().to_string(),
    }
}

pub fn lock_now() {
    let (_, _, final_output_path) = get_lock_paths();
    let wallpaper = get_wallpaper_path();

    let image_to_lock = if Path::new(&final_output_path).exists() {
        let is_recent = fs::metadata(&final_output_path)
            .and_then(|m| m.modified())
            .map(|t| t.elapsed().unwrap_or_default() < Duration::from_secs(180))
            .unwrap_or(false);

        if is_recent {
            final_output_path
        } else {
            // Si l'image a plus de 3 minutes (daemon arrêté), on rafraîchit immédiatement
            render_lock_screen()
        }
    } else {
        // Si l'image n'existe pas encore (1er lancement), on la génère
        let generated = render_lock_screen();
        if Path::new(&generated).exists() {
            generated
        } else {
            wallpaper.to_string_lossy().to_string()
        }
    };

    // Lancement instantané de swaylock
    let _ = Command::new("swaylock")
        .arg("-f")
        .arg("-i")
        .arg(&image_to_lock)
        .status();
}

pub fn run_daemon() {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let pid_file = format!("{}/lock_edt_daemon.pid", runtime_dir);

    // Tuer l'ancienne instance si présente
    if let Ok(old_pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(old_pid) = old_pid_str.trim().parse::<i32>() {
            let current_pid = std::process::id() as i32;
            if old_pid != current_pid {
                let _ = Command::new("kill").arg("-9").arg(old_pid.to_string()).status();
            }
        }
    }
    let _ = fs::write(&pid_file, std::process::id().to_string());

    println!("[lock_edt] Démarrage du daemon de pré-génération du lockscreen (PID {})...", std::process::id());
    
    // Génération initiale
    render_lock_screen();

    loop {
        thread::sleep(Duration::from_secs(5));
        render_lock_screen();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "--daemon" | "daemon" | "-d" => {
                run_daemon();
            }
            "--generate" | "generate" | "-g" => {
                let img = render_lock_screen();
                println!("Image générée : {}", img);
            }
            "--help" | "-h" => {
                println!("Usage: lock_edt [OPTIONS]");
                println!();
                println!("Options:");
                println!("  (aucun)       Verrouille l'écran instantanément avec swaylock");
                println!("  --daemon, -d  Exécute le service en arrière-plan (rafraîchit chaque minute)");
                println!("  --generate, -g Génère l'image composite une fois et quitte");
            }
            _ => {
                lock_now();
            }
        }
    } else {
        lock_now();
    }
}

