use std::path::Path;
use std::process::Command;

pub trait ScreenLocker {
    fn lock(&self, image_path: &str) -> Result<(), std::io::Error>;
}

pub struct SwaylockLocker {
    pub center_x: i32,
    pub center_y: i32,
    pub radius: u32,
    pub thickness: u32,
}

impl Default for SwaylockLocker {
    fn default() -> Self {
        Self {
            center_x: 960,
            center_y: 540,
            radius: 120,
            thickness: 6,
        }
    }
}

impl SwaylockLocker {
    pub fn new(center_x: i32, center_y: i32) -> Self {
        Self {
            center_x,
            center_y,
            ..Default::default()
        }
    }

    fn find_binary() -> &'static str {
        if Path::new("/home/martin/.local/bin/swaylock").exists() {
            "/home/martin/.local/bin/swaylock"
        } else {
            "swaylock"
        }
    }
}

pub fn fetch_color(label_color : &str) -> String {
    let colors = crate::Colors::load();
    colors
            .get(label_color)
            .trim()
            .trim_start_matches('#')
            .to_string()
}

impl ScreenLocker for SwaylockLocker {
    fn lock(&self, image_path: &str) -> Result<(), std::io::Error> {
        let bin = Self::find_binary();

        let inside_color = fetch_color("lock_inside_color");
        let ring_color = fetch_color("lock_ring_color");
        let key_hl_color = fetch_color("lock_key_hl_color");
        let bs_hl_color = fetch_color("lock_bs_hl_color");
        let line_color = fetch_color("lock_line_color");
        let separator_color = fetch_color("lock_separator_color");
        let text_color = fetch_color("lock_text_color");
        let text_clear_color = fetch_color("lock_text_clear_color");
        let text_ver_color = fetch_color("lock_text_ver_color");
        let text_wrong_color = fetch_color("lock_text_wrong_color");
        let ring_ver_color = fetch_color("lock_ring_ver_color");
        let ring_wrong_color = fetch_color("lock_ring_wrong_color");
        let ring_clear_color = fetch_color("lock_ring_clear_color");

        let _ = Command::new(bin)
            .arg("-f")
            .arg("-i")
            .arg(image_path)
            .arg("--clock")
            .arg("--indicator")
            .arg("--indicator-radius")
            .arg(self.radius.to_string())
            .arg("--indicator-thickness")
            .arg(self.thickness.to_string())
            .arg("--indicator-x-position")
            .arg(self.center_x.to_string())
            .arg("--indicator-y-position")
            .arg(self.center_y.to_string())
            .arg("--timestr")
            .arg("%H:%M:%S")
            .arg("--datestr")
            .arg("%a %d %b")
            .arg("--font")
            .arg("Noto Sans")
            .arg("--inside-color")
            .arg(&inside_color)
            .arg("--ring-color")
            .arg(&ring_color)
            .arg("--key-hl-color")
            .arg(&key_hl_color)
            .arg("--bs-hl-color")
            .arg(&bs_hl_color)
            .arg("--line-color")
            .arg(&line_color)
            .arg("--separator-color")
            .arg(&separator_color)
            .arg("--text-color")
            .arg(&text_color)
            .arg("--text-clear-color")
            .arg(&text_clear_color)
            .arg("--text-ver-color")
            .arg(&text_ver_color)
            .arg("--text-wrong-color")
            .arg(&text_wrong_color)
            .arg("--ring-ver-color")
            .arg(&ring_ver_color)
            .arg("--ring-wrong-color")
            .arg(&ring_wrong_color)
            .arg("--ring-clear-color")
            .arg(&ring_clear_color)
            .status()?;
        Ok(())
    }
}
