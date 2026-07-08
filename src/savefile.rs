use crate::ScoredRegressionEntrySlint;
use crate::table::TableData;
use slint::{Model, ModelRc, VecModel};
use spdlog::{error, warn};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct SaveFile {
    pub table_data: Option<TableData>,
    pub conf_settings: ConfigurationSettings,
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
            let _ = std::fs::write(format!("{}_old.bswprj", path), raw_str);
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

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct ScoredRegressionEntry {
    pub column_title: String,
    pub correct_answer: String,
    pub answers: Vec<String>,
    pub correct_points: u32,
    pub incorrect_points: u32,
}

impl Into<ScoredRegressionEntrySlint> for ScoredRegressionEntry {
    fn into(self) -> ScoredRegressionEntrySlint {
        ScoredRegressionEntrySlint {
            column_title: self.column_title.into(),
            correct_answer: self.correct_answer.into(),
            answers: ModelRc::new(VecModel::from(
                self.answers.iter().map(|s| s.into()).collect::<Vec<_>>(),
            )),
            correct_points: self.correct_points as i32,
            incorrect_points: self.incorrect_points as i32,
        }
    }
}

impl From<ScoredRegressionEntrySlint> for ScoredRegressionEntry {
    fn from(value: ScoredRegressionEntrySlint) -> Self {
        Self {
            column_title: value.column_title.to_string(),
            correct_answer: value.correct_answer.to_string(),
            answers: value.answers.iter().map(|s| s.to_string()).collect(),
            correct_points: value.correct_points as u32,
            incorrect_points: value.incorrect_points as u32,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct ConfigurationSettings {
    pub tab_selected: u32,
    pub simple_regression_column: Option<String>,
    pub scored_regression_data: Vec<ScoredRegressionEntry>,
    pub binary_regression_column: Option<String>,
}
