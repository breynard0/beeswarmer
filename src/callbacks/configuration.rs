use crate::appdata::AppState;
use crate::savefile::SaveFile;
use crate::table::TableColumnType;
use crate::{AppWindow, ConfigurationGlobal};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::sync::{Arc, Mutex};

pub fn configuration_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    let global = ui.global::<ConfigurationGlobal>();

    {
        let data = data.clone();
        global.on_get_numerical_column_names(move || {
            let mut out = vec![];
            if let Ok(handle) = data.lock() {
                let savefile = SaveFile::load_savefile(handle.save_file_path.clone());
                out = savefile
                    .table_data
                    .unwrap()
                    .columns
                    .iter()
                    .filter(|col| col.enabled && col.column_type == TableColumnType::Numerical)
                    .map(|col| col.title.clone().into())
                    .collect();
            }
            ModelRc::new(VecModel::from(out))
        })
    }

    {
        let data = data.clone();
        global.on_get_binary_column_names(move || {
            let mut out = vec![];
            if let Ok(handle) = data.lock() {
                let savefile = SaveFile::load_savefile(handle.save_file_path.clone());
                let excluded_rows = &savefile.table_data.clone().unwrap().excluded_rows;
                out = savefile
                    .table_data
                    .unwrap()
                    .columns
                    .iter()
                    .filter(|col| {
                        col.enabled
                            && col.column_type == TableColumnType::Categorical
                            && || -> bool {
                                let mut seen_strings = vec![];
                                for (idx, s) in col.column_entries.iter().enumerate() {
                                    if !seen_strings.contains(s) && !excluded_rows.contains(&(idx as u32)) {
                                        seen_strings.push(s.clone());
                                    }
                                }
                                seen_strings.len() == 2
                            }()
                    })
                    .map(|col| col.title.clone().into())
                    .collect();
            }
            ModelRc::new(VecModel::from(out))
        })
    }
}
