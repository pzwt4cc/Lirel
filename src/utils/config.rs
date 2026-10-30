use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub language: String, // "ru" | "en"
    pub theme: String,    // "system" | "light" | "dark"
    pub accent: i32,      // индекс цвета 0..=7
    pub output_dir: String,
    pub open_when_done: bool,
}

impl Default for Config {
    fn default() -> Self {
        let dir = dirs::audio_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."));
        Self {
            language: "ru".into(),
            theme: "system".into(),
            accent: 0,
            output_dir: dir.display().to_string(),
            open_when_done: false,
        }
    }
}

impl Config {
    fn path() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("lirel").join("config.json"))
    }

    pub fn load() -> Self {
        Self::path()
            .and_then(|p| fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }

    pub fn language_index(&self) -> i32 {
        if self.language == "en" { 1 } else { 0 }
    }

    pub fn theme_index(&self) -> i32 {
        match self.theme.as_str() {
            "light" => 1,
            "dark" => 2,
            _ => 0,
        }
    }

    pub fn set_language(&mut self, idx: i32) {
        self.language = if idx == 1 { "en" } else { "ru" }.into();
        self.save();
    }

    pub fn set_theme(&mut self, idx: i32) {
        self.theme = match idx {
            1 => "light",
            2 => "dark",
            _ => "system",
        }
        .into();
        self.save();
    }

    pub fn set_accent(&mut self, idx: i32) {
        self.accent = idx.clamp(0, 7);
        self.save();
    }

    pub fn set_output_dir(&mut self, dir: String) {
        self.output_dir = dir;
        self.save();
    }

    pub fn set_open_when_done(&mut self, on: bool) {
        self.open_when_done = on;
        self.save();
    }
}