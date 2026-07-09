pub mod configuration;
pub mod csv;
pub mod setup;

use crate::appdata::AppState;
use crate::callbacks::configuration::configuration_callbacks;
use crate::callbacks::csv::csv_callbacks;
use crate::callbacks::setup::setup_callbacks;
use crate::{AppStateCallbacks, AppWindow};
use slint::{ComponentHandle, Weak};
use std::sync::{Arc, Mutex};

fn sync_appdata(ui_handle: &Weak<AppWindow>, app_data: &AppState) {
    ui_handle
        .unwrap()
        .set_app_data_slint(app_data.clone().into())
}

pub fn handle_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    // ui.on_reset_data(move |idx| {});
    {
        let data = data.clone();
        ui.global::<AppStateCallbacks>()
            .on_sync_appstate(move |input| match data.lock() {
                Ok(mut data) => {
                    *data = AppState::from(input);
                }
                Err(_) => {}
            });
    }

    setup_callbacks(data, ui);
    csv_callbacks(data, ui);
    configuration_callbacks(data, ui);
}
