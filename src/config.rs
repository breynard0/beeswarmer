use spdlog::{error, info, warn};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Config {
    pub is_french: bool,
    pub is_dark: bool,
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
        match std::fs::write(
            Self::get_config_file_location(),
            toml::to_string(self).unwrap(),
        ) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to write config file: {e}")
            }
        }
    }

    pub fn load_config() -> Self {
        let raw_str = match std::fs::read_to_string(Self::get_config_file_location()) {
            Ok(str) => str,
            Err(_) => {
                warn!("Failed to read config, generating");
                let new_config = Self::default();
                new_config.save_config();
                return new_config;
            }
        };
        let maybe_parse = toml::from_str(&raw_str);
        maybe_parse.unwrap_or_else(|e| {
            warn!("Failed to parse config file, regenerating: {e}");
            let new_config = Self::default();
            new_config.save_config();
            return new_config;
        })
    }

    pub fn tweak_config<F>(func: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut config = Self::load_config();
        func(&mut config);
        config.save_config()
    }

    pub fn apply_config(&self) {
        info!("Applying config");
        match self.is_french {
            true => slint::select_bundled_translation("fr").unwrap_or_else(|err| {
                error!("Failed to load French locale: {err}");
            }),
            false => slint::select_bundled_translation("en").unwrap_or_else(|err| {
                error!("Failed to load English locale: {err}");
            }),
        }
    }
}
