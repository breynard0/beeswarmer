use crate::AppStateSlint;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub save_file_path: String,
}

impl Into<AppStateSlint> for AppState {
    fn into(self) -> AppStateSlint {
        AppStateSlint {
            save_file_path: self.save_file_path.into(),
        }
    }
}

impl From<AppStateSlint> for AppState {
    fn from(value: AppStateSlint) -> Self {
        Self {
            save_file_path: value.save_file_path.to_string(),
        }
    }
}
