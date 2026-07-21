use crate::appdata::AppState;
use crate::ml::ConfigurationLock;
use crate::ml::OutputColumnData::{BinaryClassificatory, Regressive};
use crate::savefile::{SaveFile, ScoredRegressionEntry};
use crate::table::TableColumnType;
use crate::{AppWindow, ConfigurationGlobal, ScoredRegressionEntrySlint};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use spdlog::{error, warn};
use std::sync::{Arc, Mutex, MutexGuard};

//noinspection DuplicatedCode
pub fn configuration_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    let global = ui.global::<ConfigurationGlobal>();
    fn get_unset_with_language(data: &MutexGuard<AppState>) -> slint::SharedString {
        match data.french_selected {
            true => "Vide",
            false => "Unset",
        }
        .into()
    }
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
                out.insert(0, get_unset_with_language(&handle))
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
                out.insert(0, get_unset_with_language(&handle))
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
                out.insert(0, get_unset_with_language(&handle))
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
                if out == "Unset".to_string() && handle.french_selected {
                    out = "Vide".to_string();
                }
                if out == "Vide".to_string() && !handle.french_selected {
                    out = "Unset".to_string();
                }
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
                let table_data = &save_file.table_data.unwrap();
                out = save_file
                    .conf_settings
                    .scored_regression_data
                    .iter()
                    .filter(|s| {
                        match table_data
                            .columns
                            .iter()
                            .find(|col| col.title == s.column_title)
                        {
                            Some(col) => match col.enabled {
                                true => true,
                                false => {
                                    remove_scored_regression_entry(
                                        handle.save_file_path.clone(),
                                        s,
                                    );
                                    false
                                }
                            },
                            None => {
                                remove_scored_regression_entry(handle.save_file_path.clone(), s);
                                false
                            }
                        }
                    })
                    .map(|x| {
                        let mut out = x.clone();

                        if out.column_title == "Unset".to_string() && handle.french_selected {
                            out.column_title = "Vide".to_string();
                        }
                        if out.column_title == "Vide".to_string() && !handle.french_selected {
                            out.column_title = "Unset".to_string();
                        }

                        out
                    })
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
                if out == "Unset".to_string() && handle.french_selected {
                    out = "Vide".to_string();
                }
                if out == "Vide".to_string() && !handle.french_selected {
                    out = "Unset".to_string();
                }
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

    {
        global.on_check_contains_zero_entry(|array| {
            array
                .iter()
                .find(|x| x.correct_points == 0 && x.incorrect_points == 0)
                .is_some()
        })
    }

    {
        let data = data.clone();
        global.on_conf_continue(move || {
            if let Ok(handle) = data.lock() {
                let mut save_file = SaveFile::load_savefile(handle.save_file_path.clone());

                let table = save_file.table_data.clone().unwrap();
                let conf_settings = save_file.conf_settings.clone();

                let output_data;
                let output_name = match conf_settings.tab_selected {
                    0 => {
                        let col = table.columns
                            .iter()
                            .find(|c| c.title == conf_settings.simple_regression_column.clone().unwrap()
                                && c.column_type == TableColumnType::Numerical)
                            .unwrap()
                            .column_entries
                            .iter()
                            .map(|s| s.parse().unwrap())
                            .collect::<Vec<_>>();
                        output_data = Regressive(col);
                        conf_settings.simple_regression_column.unwrap()
                    }
                    1 => {
                        let mut score_rows = vec![];
                        for index in 0..table.number_rows {
                            if table.excluded_rows.contains(&index) { continue; }

                            let mut score: f64 = 0.0;
                            for question in &conf_settings.scored_regression_data {
                                let value = &table.columns
                                    .iter()
                                    .find(|x| x.title == question.column_title)
                                    .unwrap()
                                    .column_entries[index as usize];
                                if *value == question.correct_answer {
                                    score += question.correct_points as f64;
                                } else {
                                    score -= question.incorrect_points as f64;
                                }
                            }
                            score_rows.push(score);
                        }
                        output_data = Regressive(score_rows);
                        "score".to_string()
                    }
                    2 => {
                        let col = table.columns
                            .iter()
                            .find(|c| c.title == conf_settings.binary_regression_column.clone().unwrap()
                                && c.column_type == TableColumnType::Categorical)
                            .unwrap()
                            .column_entries
                            .iter()
                            .map(|s| s.parse().unwrap())
                            .collect::<Vec<_>>();
                        output_data = BinaryClassificatory(col);
                        conf_settings.binary_regression_column.unwrap()
                    }
                    _ => {
                        error!("User somehow managed to select a non-existent tab in Configuration view. Congratulations.");
                        return;
                    }
                };

                save_file.conf_lock = Some(ConfigurationLock {
                    numerical_columns: table.columns
                        .iter()
                        .filter(|col| col.enabled && col.column_type == TableColumnType::Numerical && col.title != output_name)
                        .map(|col| (
                            col.title.clone(),
                            col.column_entries
                                .iter()
                                .enumerate()
                                .filter(|(x, _)| !table.excluded_rows.contains(&(*x as u32)))
                                .map(|(_, x)| x.parse::<f64>().unwrap())
                                .collect()
                        )
                        ).collect(),
                    categorical_columns: table.columns
                        .iter()
                        .filter(|col| col.enabled
                            && col.column_type == TableColumnType::Categorical
                            && col.title != output_name
                            && conf_settings.scored_regression_data
                            .iter()
                            .find(|x| x.column_title == col.title)
                            .is_none()
                        )
                        .map(|col| (
                            col.title.clone(),
                            col.column_entries
                                .iter()
                                .enumerate()
                                .filter(|(x, _)| !table.excluded_rows.contains(&(*x as u32)))
                                .map(|(_, s)| s.clone())
                                .collect()
                        )
                        ).collect(),
                    output_name,
                    output_data,
                });

                save_file.save_savefile(handle.save_file_path.clone());
            }
        });
    }
}

fn remove_scored_regression_entry(save_file_path: String, s: &&ScoredRegressionEntry) {
    SaveFile::tweak_savefile(save_file_path, |savefile| {
        savefile.conf_settings.scored_regression_data.remove(
            savefile
                .conf_settings
                .scored_regression_data
                .iter()
                .position(|x| x.column_title == s.column_title)
                .unwrap(),
        );
    });
}
