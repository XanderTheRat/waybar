use crate::edt::Edt;
use crate::lock::content::CardContent;
use crate::lock::renderer::SvgCardRenderer;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

pub struct LockPaths {
    pub svg: String,
    pub tmp_output: String,
    pub final_output: String,
}

impl Default for LockPaths {
    fn default() -> Self {
        let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            svg: format!("{}/swaylock_edt.svg", runtime_dir),
            tmp_output: format!("{}/swaylock_lock.tmp.png", runtime_dir),
            final_output: format!("{}/swaylock_lock.png", runtime_dir),
        }
    }
}

pub struct LockImageGenerator {
    paths: LockPaths,
    renderer: SvgCardRenderer,
}

impl Default for LockImageGenerator {
    fn default() -> Self {
        Self {
            paths: LockPaths::default(),
            renderer: SvgCardRenderer::default(),
        }
    }
}

impl LockImageGenerator {
    pub fn new(paths: LockPaths, renderer: SvgCardRenderer) -> Self {
        Self { paths, renderer }
    }

    pub fn paths(&self) -> &LockPaths {
        &self.paths
    }

    pub fn find_wallpaper() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/martin".to_string());
        let candidates = ["outerwilds2.jpg", "outerwilds1.jpg"];
        for candidate in candidates {
            let p = PathBuf::from(&home).join("wallpaper").join(candidate);
            if p.exists() {
                return p;
            }
        }
        PathBuf::from(&home).join("wallpaper/outerwilds2.jpg")
    }

    pub fn render(&self) -> String {
        let wallpaper = Self::find_wallpaper();
        let edt = Edt::default();
        let schedule_res = edt.fetch_or_cached(Duration::from_secs(300));
        let content = CardContent::from_schedule_result(schedule_res);
        let svg_content = self.renderer.render(&content);

        let previous_svg = fs::read_to_string(&self.paths.svg).unwrap_or_default();
        if previous_svg == svg_content && Path::new(&self.paths.final_output).exists() {
            return self.paths.final_output.clone();
        }

        let _ = fs::write(&self.paths.svg, &svg_content);

        let status = Command::new("magick")
            .arg(&wallpaper)
            .arg("(")
            .arg("-background")
            .arg("none")
            .arg(&self.paths.svg)
            .arg(")")
            .arg("-composite")
            .arg(&self.paths.tmp_output)
            .status();

        match status {
            Ok(s) if s.success() && Path::new(&self.paths.tmp_output).exists() => {
                let _ = fs::rename(&self.paths.tmp_output, &self.paths.final_output);
                self.paths.final_output.clone()
            }
            _ => wallpaper.to_string_lossy().to_string(),
        }
    }

    pub fn get_or_render_recent(&self, max_age: Duration) -> String {
        if Path::new(&self.paths.final_output).exists() {
            let is_recent = fs::metadata(&self.paths.final_output)
                .and_then(|m| m.modified())
                .map(|t| t.elapsed().unwrap_or_default() < max_age)
                .unwrap_or(false);

            if is_recent {
                return self.paths.final_output.clone();
            }
        }
        self.render()
    }
}
