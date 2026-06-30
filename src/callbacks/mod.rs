use crate::AppWindow;
use crate::appdata::AppState;
use crate::config::Config;
use slint::{ComponentHandle, Weak};
use spdlog::info;
use std::sync::{Arc, Mutex};

fn sync_appdata(ui_handle: &Weak<AppWindow>, app_data: &AppState) {
    ui_handle
        .unwrap()
        .set_app_data_slint(app_data.clone().into())
}

pub fn handle_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    // ui.on_reset_data(move |idx| {});
    {
        let data = data.clone();
        ui.on_file_browser(move |create_new| {
            match data.lock() {
                Ok(mut data) => {
                    if create_new {
                        info!("Opening file save dialog");
                        let result = rfd::FileDialog::new()
                            .add_filter("Beeswarmer Project", &["bswproj"])
                            .save_file();
                        if result.is_some() {
                            data.save_file_path = result.unwrap().to_str().unwrap().to_string();

                            // Append .bswproj if does not exist
                            let extension = data
                                .save_file_path
                                .chars()
                                .rev()
                                .take(".bswproj".len())
                                .collect::<String>();

                            if extension.chars().rev().collect::<String>() != ".bswproj" {
                                info!(
                                    "Provided file name (\'{}\') did not include extension, adding now",
                                    data.save_file_path
                                );
                                data.save_file_path.push_str(".bswproj");
                            }

                            info!("New save file set: \'{}\'", data.save_file_path);
                        }
                    } else {
                        info!("Opening file picker dialog");
                        let result = rfd::FileDialog::new()
                            .add_filter("Beeswarmer Project", &["bswproj"])
                            .pick_file();
                        if result.is_some() {
                            data.save_file_path = result.unwrap().to_str().unwrap().to_string();
                            info!("New save file set: \'{}\'", data.save_file_path);
                        }
                    }
                    sync_appdata(&ui_handle, &data);
                }
                Err(_) => {}
            };
        });
    }

    {
        ui.on_language_changed(|is_french| {
            let mut c = Config::load_config();
            c.is_french = is_french;
            c.apply_config();
            c.save_config();
        });
    }
}
