pub mod edt;
pub mod lock;

pub use edt::Edt;

use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Default)]
pub struct Colors {
    map: HashMap<String, String>,
}

impl Colors {
    pub fn load() -> Self {
        Self {
            map: load_colors(),
        }
    }

    pub fn get(&self, key: &str) -> String {
        self.map
            .get(&key.to_lowercase())
            .cloned()
            .unwrap_or_default()
    }
}

pub fn load_colors() -> HashMap<String, String> {
    let mut map = HashMap::new();
    let candidates = [
        std::env::var("WAYBAR_THEME_CSS").ok(),
        std::env::var("WAYBAR_COLORS_CONF").ok(),
        Some("../theme.css".to_string()),
        std::env::var("HOME")
            .ok()
            .map(|h| format!("{}/.config/waybar/theme.css", h)),
        Some("theme.css".to_string()),
        Some("/home/martin/.config/waybar/theme.css".to_string()),
        std::env::var("HOME")
            .ok()
            .map(|h| format!("{}/.config/waybar/scripts/colors.conf", h)),
        Some("colors.conf".to_string()),
        Some("/home/martin/.config/waybar/scripts/colors.conf".to_string()),
    ];

    for candidate in candidates.into_iter().flatten() {
        if let Ok(content) = fs::read_to_string(&candidate) {
            let mut current_section = String::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty()
                    || trimmed.starts_with('#')
                    || trimmed.starts_with(';')
                    || trimmed.starts_with("/*")
                    || trimmed.starts_with("//")
                {
                    continue;
                }

                if let Some(rest) = trimmed.strip_prefix("@define-color") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let key = parts[0].trim().to_lowercase();
                        let val = parts[1..].join(" ");
                        let val_str = val
                            .trim()
                            .trim_end_matches(';')
                            .trim_matches('"')
                            .trim_matches('\'')
                            .trim()
                            .to_string();
                        map.insert(key, val_str);
                    }
                    continue;
                }

                if trimmed.starts_with('[') && trimmed.ends_with(']') {
                    current_section = trimmed[1..trimmed.len() - 1].trim().to_lowercase();
                    continue;
                }

                if let Some((k, v)) = trimmed.split_once('=') {
                    let key = k.trim().to_lowercase();
                    let mut val = v.trim();
                    if let Some(idx) = val.find(';') {
                        val = val[..idx].trim();
                    }
                    let val_str = val
                        .trim_matches('"')
                        .trim_matches('\'')
                        .trim()
                        .to_string();
                    if !current_section.is_empty() {
                        map.insert(format!("{}_{}", current_section, key), val_str.clone());
                        map.insert(format!("{}.{}", current_section, key), val_str.clone());
                    }
                    map.insert(key, val_str);
                } else if let Some((k, v)) = trimmed.split_once(':') {
                    let key = k.trim().trim_start_matches('-').to_lowercase();
                    let val = v
                        .trim()
                        .trim_end_matches(';')
                        .trim_matches('"')
                        .trim_matches('\'')
                        .trim()
                        .to_string();
                    map.insert(key, val);
                }
            }
            break;
        }
    }
    map
}

