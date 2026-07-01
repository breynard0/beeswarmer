use crate::appdata::AppState;
use crate::callbacks::sync_appdata;
use crate::config::Config;
use crate::{AppWindow, SetupCallbacks};
use slint::{ComponentHandle, ModelRc, VecModel};
use spdlog::{error, info, warn};
use std::sync::{Arc, Mutex};

pub fn setup_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    let global = ui.global::<SetupCallbacks>();
    {
        let data = data.clone();
        global.on_file_browser(move |create_new| {
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

                            match std::fs::write(&data.save_file_path, vec![]) {
                                Ok(_) => info!("New save file created"),
                                Err(e) => error!("Failed to write save file: {}", e)
                            };
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
        global.on_language_changed(move |is_french| {
            let mut c = Config::load_config();
            c.is_french = is_french;
            c.apply_config();
            c.save_config();
        });
    }

    {
        global.on_theme_changed(move |dark| {
            let mut c = Config::load_config();
            c.is_dark = dark;
            c.apply_config();
            c.save_config();
        });
    }

    {
        let data = data.clone();
        global.on_setup_next_button(move || {
            let mut output = String::new();
            if let Ok(data) = data.lock() {
                let path = &data.save_file_path;
                match std::fs::exists(path) {
                    Ok(res) => {
                        if res {
                            Config::tweak_config(|conf| {
                                if conf.recent_projects.contains(path) {
                                    conf.recent_projects.remove(
                                        conf.recent_projects
                                            .iter()
                                            .position(|x| x == path)
                                            .unwrap(),
                                    );
                                }
                                conf.recent_projects.insert(0, path.clone());
                            });
                            return "yay".into();
                        } else {
                            output = match data.french_selected {
                                true => format!("Fichier des données pas trouvé à {path}"),
                                false => format!("Save file not found at provided path {path}"),
                            };
                        }
                    }
                    Err(e) => {
                        output = match data.french_selected {
                            true => {
                                format!("Ne peut pas vérifier chemin de fichier, {e}").to_string()
                            }
                            false => format!("Could not verify file path, {e}").to_string(),
                        }
                    }
                };
                if output.is_empty() {
                    output = match data.french_selected {
                        true => "Ne pouvais pas barrer systeme de données interne".to_string(),
                        false => "Failed to gain lock on internal data pipeline".to_string(),
                    }
                }
            };
            warn!("{output}");
            return output.into();
        })
    }

    {
        global.on_get_recent_files(|| {
            ModelRc::new(VecModel::from(
                Config::load_config()
                    .recent_projects
                    .iter()
                    .map(|x| x.into())
                    .collect::<Vec<slint::SharedString>>(),
            ))
        })
    }
}
