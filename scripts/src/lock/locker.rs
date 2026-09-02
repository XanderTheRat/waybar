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

impl ScreenLocker for SwaylockLocker {
    fn lock(&self, image_path: &str) -> Result<(), std::io::Error> {
        let bin = Self::find_binary();
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
            .arg("0d1b2acc")
            .arg("--ring-color")
            .arg("38bdf855")
            .arg("--key-hl-color")
            .arg("38bdf8")
            .arg("--bs-hl-color")
            .arg("f472b6")
            .arg("--line-color")
            .arg("00000000")
            .arg("--separator-color")
            .arg("00000000")
            .arg("--text-color")
            .arg("ffffff")
            .arg("--text-clear-color")
            .arg("ffffff")
            .arg("--text-ver-color")
            .arg("38bdf8")
            .arg("--text-wrong-color")
            .arg("f87171")
            .arg("--ring-ver-color")
            .arg("38bdf8")
            .arg("--ring-wrong-color")
            .arg("f87171")
            .arg("--ring-clear-color")
            .arg("a78bfa")
            .status()?;
        Ok(())
    }
}
