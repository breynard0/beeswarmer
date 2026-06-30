use crate::AppStateSlint;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub save_file_path: String,
    pub french_selected: bool,
}

impl Into<AppStateSlint> for AppState {
    fn into(self) -> AppStateSlint {
        AppStateSlint {
            french_selected: self.french_selected,
            save_file_path: self.save_file_path.into(),
        }
    }
}

impl From<AppStateSlint> for AppState {
    fn from(value: AppStateSlint) -> Self {
        Self {
            save_file_path: value.save_file_path.to_string(),
            french_selected: value.french_selected,
        }
    }
}
