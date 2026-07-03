use crate::appdata::AppState;
use crate::savefile::SaveFile;
use crate::table::{TableColumn, TableColumnType, TableData};
use crate::{AppWindow, CSVGlobal, TableDataSlint};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use spdlog::{error, warn};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn csv_callbacks(_data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    // let ui_handle = ui.as_weak();

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
        global.on_import_csv(|| {
            let result = rfd::FileDialog::new()
                .add_filter("CSV Data File", &["csv"])
                .pick_file();
            if let None = result {
                warn!("No file selected");
                return TableDataSlint::default();
            }

            let read = std::fs::read_to_string(result.unwrap());
            if let Err(e) = read {
                error!("Failed to read file: {e}");
                return TableDataSlint::default();
            }

            let parse = TableData::from_csv(read.unwrap());
            if let Err(e) = parse {
                error!("Failed to parse CSV data: {e}");
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

            let mut all_set = true;
            table.columns.iter().for_each(|column| {
                column
                    .column_entries
                    .iter()
                    .enumerate()
                    .for_each(|(idx, s)| {
                        if s.is_empty() && !table.excluded_rows.contains(&(idx as u32)) {
                            all_set = false;
                        }
                    })
            });

            all_set
        })
    }

    {
        global.on_check_all_columns_set(|table_data_slint| {
            let columns = table_data_slint
                .columns
                .iter()
                .map(|x| TableColumn::from(x))
                .collect::<Vec<_>>();
            let out = columns
                .iter()
                .find(|col| col.column_type == TableColumnType::Unset && col.enabled)
                .is_none();
            out
        })
    }
}
