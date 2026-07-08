use crate::appdata::AppState;
use crate::savefile::{SaveFile, ScoredRegressionEntry};
use crate::table::TableColumnType;
use crate::{AppWindow, ConfigurationGlobal, ScoredRegressionEntrySlint};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use spdlog::warn;
use std::sync::{Arc, Mutex};
//noinspection DuplicatedCode
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
                out.insert(0, "Unset".into())
            }
            ModelRc::new(VecModel::from(out))
        })
    }

    {
        let data = data.clone();
        global.on_get_categorical_column_names(move || {
            let mut out = vec![];
            if let Ok(handle) = data.lock() {
                let savefile = SaveFile::load_savefile(handle.save_file_path.clone());
                out = savefile
                    .table_data
                    .unwrap()
                    .columns
                    .iter()
                    .filter(|col| col.enabled && col.column_type == TableColumnType::Categorical)
                    .map(|col| col.title.clone().into())
                    .collect();
                out.insert(0, "Unset".into())
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
                                    if !seen_strings.contains(s)
                                        && !excluded_rows.contains(&(idx as u32))
                                    {
                                        seen_strings.push(s.clone());
                                    }
                                }
                                seen_strings.len() == 2
                            }()
                    })
                    .map(|col| col.title.clone().into())
                    .collect();
                out.insert(0, "Unset".into())
            }
            ModelRc::new(VecModel::from(out))
        })
    }

    {
        let data = data.clone();
        global.on_load_simple_selected(move || {
            let mut out = String::new();
            if let Ok(handle) = data.lock() {
                let save_file = SaveFile::load_savefile(handle.save_file_path.clone());
                out = save_file
                    .conf_settings
                    .simple_regression_column
                    .unwrap_or_else(|| "Unset".to_string());
            }
            out.into()
        })
    }

    {
        let data = data.clone();
        global.on_load_scored_regression_entries(move || {
            let mut out = vec![];
            if let Ok(handle) = data.lock() {
                let save_file = SaveFile::load_savefile(handle.save_file_path.clone());
                out = save_file
                    .conf_settings
                    .scored_regression_data
                    .iter()
                    .map(|x| x.clone().into())
                    .collect::<Vec<_>>()
            }
            ModelRc::new(VecModel::from(out))
        })
    }

    {
        let data = data.clone();
        global.on_load_binary_selected(move || {
            let mut out = String::new();
            if let Ok(handle) = data.lock() {
                let save_file = SaveFile::load_savefile(handle.save_file_path.clone());
                out = save_file
                    .conf_settings
                    .binary_regression_column
                    .unwrap_or_else(|| "Unset".to_string());
            }
            out.into()
        })
    }

    {
        let data = data.clone();
        global.on_load_tab_selected(move || {
            let mut out = 0;
            if let Ok(handle) = data.lock() {
                let save_file = SaveFile::load_savefile(handle.save_file_path.clone());
                out = save_file.conf_settings.tab_selected as i32
            }
            out
        })
    }

    {
        let data = data.clone();
        global.on_save_simple_selected(move |s| {
            if let Ok(handle) = data.lock() {
                SaveFile::tweak_savefile(handle.save_file_path.clone(), |savefile| {
                    savefile.conf_settings.simple_regression_column = Some(s.into());
                });
            }
        })
    }

    {
        let data = data.clone();
        global.on_save_scored_regression_entries(move |s| {
            if let Ok(handle) = data.lock() {
                SaveFile::tweak_savefile(handle.save_file_path.clone(), |savefile| {
                    savefile.conf_settings.scored_regression_data = s
                        .iter()
                        .map(|x| ScoredRegressionEntry::from(x))
                        .collect::<Vec<_>>();
                });
            }
        })
    }

    {
        let data = data.clone();
        global.on_save_binary_selected(move |s| {
            if let Ok(handle) = data.lock() {
                SaveFile::tweak_savefile(handle.save_file_path.clone(), |savefile| {
                    savefile.conf_settings.binary_regression_column = Some(s.into());
                });
            }
        })
    }

    {
        let data = data.clone();
        global.on_save_tab_selected(move |idx| {
            if let Ok(handle) = data.lock() {
                SaveFile::tweak_savefile(handle.save_file_path.clone(), |savefile| {
                    savefile.conf_settings.tab_selected = idx as u32;
                });
            }
        })
    }

    {
        let data = data.clone();
        global.on_add_scored_regression_entry(move |entry_title, array| {
            let mut v = array.iter().collect::<Vec<_>>();
            if array
                .iter()
                .find(|s| s.column_title == entry_title)
                .is_none()
            {
                if let Ok(handle) = data.lock() {
                    let save_file = SaveFile::load_savefile(handle.save_file_path.clone());
                    let table_data = save_file.table_data.unwrap();
                    let column_option = table_data
                        .columns
                        .iter()
                        .find(|x| x.title == entry_title.to_string());
                    if let Some(column) = column_option {
                        let mut entries = column.column_entries.clone();
                        entries.sort();
                        entries.dedup();
                        let answers: Vec<slint::SharedString> =
                            entries.iter().map(|s| s.into()).collect::<Vec<_>>();
                        v.push(ScoredRegressionEntrySlint {
                            answers: ModelRc::new(VecModel::from(answers)),
                            correct_answer: entries[0].clone().into(),
                            column_title: entry_title,
                            correct_points: 0,
                            incorrect_points: 0,
                        });
                    }
                }
            }
            ModelRc::new(VecModel::from(v))
        })
    }

    {
        global.on_remove_scored_regression_entry(|name, array| {
            let mut v = array.iter().collect::<Vec<_>>();
            let position = v.iter().position(|s| s.column_title == name);
            match position {
                Some(idx) => {
                    let _ = v.remove(idx);
                }
                None => warn!("Failed to remove scored regression entry, bad title given"),
            }
            ModelRc::new(VecModel::from(v))
        })
    }
}
