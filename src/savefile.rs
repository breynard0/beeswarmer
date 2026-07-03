use crate::table::TableData;
use spdlog::{error, warn};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SaveFile {
    pub table_data: Option<TableData>,
}

impl SaveFile {
    pub fn save_savefile(&self, path: String) {
        match std::fs::write(path, toml::to_string(self).unwrap()) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to write config file: {e}")
            }
        }
    }

    pub fn load_savefile(path: String) -> Self {
        let raw_str = match std::fs::read_to_string(&path) {
            Ok(str) => str,
            Err(_) => {
                warn!("Failed to read save file, generating");
                let new_savefile = Self::default();
                new_savefile.save_savefile(path);
                return new_savefile;
            }
        };
        let maybe_parse = toml::from_str(&raw_str);
        maybe_parse.unwrap_or_else(|e| {
            warn!("Failed to parse save file, regenerating: {e}");
            let new_savefile = Self::default();
            new_savefile.save_savefile(path);
            return new_savefile;
        })
    }

    pub fn tweak_savefile<F>(path: String, func: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut savefile = Self::load_savefile(path.clone());
        func(&mut savefile);
        savefile.save_savefile(path)
    }
}
