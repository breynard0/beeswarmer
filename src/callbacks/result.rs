use crate::appdata::AppState;
use crate::ml::model::generate_pixmap_from_svg;
use crate::savefile::SaveFile;
use crate::{AppWindow, ResultsGlobal};
use image::RgbaImage;
use slint::ComponentHandle;
use spdlog::warn;
use std::io::Cursor;
use std::sync::{Arc, Mutex};

// Avert your eyes from this terrible bit of code.
// Since this gets called exactly once, I think I can get away without a mutex.
// Please don't call this more than once
static CURRENT_IMAGE_SVG: Mutex<Option<String>> = Mutex::new(None);
pub fn get_current_preview_image() -> Option<String> {
    CURRENT_IMAGE_SVG.lock().unwrap().to_owned()
}
pub fn set_current_preview_image(img: String) {
    *CURRENT_IMAGE_SVG.lock().unwrap() = Some(img);
}

pub fn fix_hex<S>(hex: &S) -> String
where
    S: ToString,
{
    let s = hex.to_string();
    if s.len() == 0 {
        return s;
    }
    if s.chars().nth(0).unwrap() != '#' {
        format!("#{}", s)
    } else {
        s
    }
}

pub fn result_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    let global = ui.global::<ResultsGlobal>();
    {
        let data = data.clone();
        let ui_handle = ui.as_weak();

        global.on_gen_beeswarm_img(move |theme| {
            let data = data.clone();
            let ui_handle = ui_handle.clone();
            if let Ok(data) = data.lock() {
                let savefile = SaveFile::load_savefile(data.save_file_path.clone());
                let lock = savefile.conf_lock;
                if lock.is_none() {
                    return;
                }
                crate::ml::model::gen_model(lock.unwrap(), theme, ui_handle);
            }
        });

        global.on_check_hex_correct(|s| piet::Color::from_hex_str(&s).is_ok());
    }
    {
        let ui_handle = ui.as_weak();
        global.on_save_image(move |extension| {
            let svg = match get_current_preview_image() {
                Some(s) => s,
                None => {
                    warn!("Could not export image, singleton is empty");
                    return;
                }
            };

            let ui_handle = ui_handle.clone();
            let _ = std::thread::spawn(move || {
                let file_path_option = rfd::FileDialog::new()
                    .add_filter(
                        format!("{} File", extension.to_uppercase()),
                        &[extension.as_str()],
                    )
                    .set_can_create_directories(true)
                    .save_file();
                if let Some(path_buf) = file_path_option {
                    let mut bytes = vec![];
                    match extension.to_string().as_str() {
                        "png" => {
                            let pixmap = generate_pixmap_from_svg(&svg);
                            let img = RgbaImage::from_raw(
                                pixmap.width(),
                                pixmap.height(),
                                pixmap.data().into(),
                            )
                            .unwrap();
                            let _ =
                                img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png);
                        }
                        "svg" => {
                            bytes = svg.into_bytes();
                        }
                        _ => {
                            warn!("Could not export image, unrecognized extension");
                        }
                    }
                    let mut path = path_buf.to_string_lossy().to_string();
                    if !path.ends_with(extension.as_str()) {
                        path.push_str(&format!(".{}", extension));
                    }
                    let _ = std::fs::write(path, bytes);
                    let _ = ui_handle.upgrade_in_event_loop(move |ui| {
                        ui.global::<ResultsGlobal>().set_throbber_shown(false);
                    });
                }
            });
        });
    }
}
