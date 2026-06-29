use spdlog::error;
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub is_french: bool,
    pub recent_projects: Vec<String>,
}

#[cfg(target_os = "windows")]
const SLASH: char = '\\';
#[cfg(not(target_os = "windows"))]
const SLASH: char = '/';

impl Config {
    pub fn get_config_file_location() -> String {
        if let Some(dirs) = directories::BaseDirs::new() {
            format!(
                "{}{SLASH}beeswarmer.toml",
                dirs.config_local_dir().to_string_lossy().to_string()
            )
        } else {
            error!("Failed to load config directory");
            String::new()
        }
    }

    pub fn save_config(&self) {
        // std::fs::write(Self::get_config_file_location(), toml)
    }
}
