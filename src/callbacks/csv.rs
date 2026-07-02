use crate::appdata::AppState;
use crate::table::TableData;
use crate::{AppWindow, CSVGlobal, TableDataSlint};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use spdlog::{error, warn};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, LockResult, Mutex};

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
}
