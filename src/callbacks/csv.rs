use crate::appdata::AppState;
use crate::{AppWindow, CSVGlobal};
use slint::{ComponentHandle, Model, ModelRc, VecModel};
use spdlog::warn;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn csv_callbacks(_data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    // let ui_handle = ui.as_weak();

    let global = ui.global::<CSVGlobal>();

    {
        // global.on_sync_data_table(|table| {});
    }

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
}
