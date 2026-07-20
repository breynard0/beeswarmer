use crate::appdata::AppState;
use crate::savefile::SaveFile;
use crate::table::{TableColumn, TableColumnType, TableData};
use crate::{AppWindow, CSVGlobal, CheckResultSlint, TableDataSlint};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use spdlog::{error, warn};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

pub fn csv_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    let global = ui.global::<CSVGlobal>();

    {
        global.on_get_mutated_excluded_rows(|array, checked, row| {
            let vec_model_ref = array.as_any().downcast_ref::<VecModel<i32>>();
            let mut vec;
            if let None = vec_model_ref {
                warn!("Failed to parse ModelRc into VecModel");
                vec = vec![]
            } else {
                vec = vec_model_ref.unwrap().iter().collect::<Vec<_>>();
            }

            if checked {
                // Remove from excluded
                if vec.iter().find(|x| **x == row).is_some() {
                    vec.remove(vec.iter().position(|x| *x == row).unwrap());
                }
            } else {
                vec.push(row);
            }

            ModelRc::from(Rc::new(VecModel::from(vec)))
        });
    }

    {
        global.on_table_contains(|array, row| {
            let vec_model_ref = array.as_any().downcast_ref::<VecModel<i32>>();
            let vec;
            if let None = vec_model_ref {
                warn!("Failed to parse ModelRc into VecModel");
                vec = vec![];
            } else {
                vec = vec_model_ref.unwrap().iter().collect::<Vec<_>>();
            }
            return vec.contains(&row);
        });
    }

    {
        let data = data.clone();
        global.on_import_csv(move || {
            let result = rfd::FileDialog::new()
                .add_filter("CSV Data File", &["csv"])
                .pick_file();
            if let None = result {
                warn!("No file selected");
                if let Ok(handle) = data.try_lock() {
                    if let Some(value) = return_table_if_exists(handle) {
                        return value;
                    }
                }
                return TableDataSlint::default();
            }

            let read = std::fs::read_to_string(result.unwrap());
            if let Err(e) = read {
                error!("Failed to read file: {e}");
                if let Ok(handle) = data.try_lock() {
                    if let Some(value) = return_table_if_exists(handle) {
                        return value;
                    }
                }
                return TableDataSlint::default();
            }

            let parse = TableData::from_csv(read.unwrap());
            if let Err(e) = parse {
                error!("Failed to parse CSV data: {e}");
                if let Ok(handle) = data.try_lock() {
                    if let Some(value) = return_table_if_exists(handle) {
                        return value;
                    }
                }
                return TableDataSlint::default();
            }

            return parse.unwrap().into();
        });
    }

    {
        global.on_save_table(move |table, path| {
            if path.is_empty() {
                error!("No save file path found in CSV Editor stage");
                return;
            }

            SaveFile::tweak_savefile(path.into(), move |savefile| {
                savefile.table_data = Some(table.into());
            })
        });
    }

    {
        global.on_check_all_cells_have_value(|table_data_slint| {
            let table = TableData::from(table_data_slint);

            let mut all_correct = true;
            let mut x = -1;
            let mut y = -1;
            table
                .columns
                .iter()
                .enumerate()
                .for_each(|(column_x, column)| {
                    column
                        .column_entries
                        .iter()
                        .enumerate()
                        .for_each(|(idx, s)| {
                            if all_correct {
                                if s.is_empty()
                                    && !table.excluded_rows.contains(&(idx as u32))
                                    && column.enabled
                                {
                                    all_correct = false;
                                    x = column_x as i32;
                                    y = idx as i32;
                                }
                            }
                        })
                });

            CheckResultSlint {
                ok: all_correct,
                x,
                y,
            }
        })
    }

    {
        global.on_check_all_columns_set(|table_data_slint| {
            let mut x = -1;
            let columns = table_data_slint
                .columns
                .iter()
                .map(|x| TableColumn::from(x))
                .collect::<Vec<_>>();
            let out_type = columns
                .iter()
                .find(|col| col.column_type == TableColumnType::Unset && col.enabled)
                .is_none();
            if out_type == false {
                x = columns
                    .iter()
                    .position(|col| col.column_type == TableColumnType::Unset && col.enabled)
                    .unwrap() as i32;
            }
            let out_name = table_data_slint
                .columns
                .iter()
                .find(|x| x.title.is_empty() && x.enabled)
                .is_none();
            if out_name == false {
                x = table_data_slint
                    .columns
                    .iter()
                    .position(|x| x.title.is_empty() && x.enabled)
                    .unwrap() as i32;
            }
            CheckResultSlint {
                ok: out_type && out_name,
                x,
                y: -1,
            }
        })
    }

    {
        global.on_check_numerical_check(|table_data_slint| {
            let data = TableData::from(table_data_slint);
            let mut all_correct = true;
            let mut x = -1;
            let mut y = -1;
            for (column_idx, col) in data.columns.iter().enumerate() {
                if !col.enabled {
                    continue;
                }
                if let TableColumnType::Numerical = col.column_type {
                    col.column_entries.iter().enumerate().for_each(|(idx, s)| {
                        let res: Result<f64, _> = s.parse();
                        if res.is_err() && !data.excluded_rows.contains(&(idx as u32)) {
                            all_correct = false;
                            x = column_idx as i32;
                            y = idx as i32;
                        }
                    })
                }
            }
            CheckResultSlint {
                ok: all_correct,
                x,
                y,
            }
        })
    }

    {
        global.on_check_table_empty(|table_data_slint| CheckResultSlint {
            ok: table_data_slint.number_rows != 0 && table_data_slint.columns.iter().len() != 0,
            x: -1,
            y: -1,
        })
    }
}

fn return_table_if_exists(handle: MutexGuard<AppState>) -> Option<TableDataSlint> {
    if let Some(out) = SaveFile::load_savefile(handle.save_file_path.clone()).table_data {
        return Some(out.into());
    }
    None
}
