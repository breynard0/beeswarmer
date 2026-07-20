use crate::appdata::AppState;
use crate::savefile::SaveFile;
use crate::{AppWindow, ResultsGlobal};
use slint::{ComponentHandle, Image};
use std::sync::{Arc, Mutex};

// Avert your eyes from this terrible bit of code.
// Since this gets called exactly once, I think I can get away without a mutex.
// Please don't call this more than once
static mut CURRENT_IMAGE: Option<Image> = None;
pub unsafe fn set_current_preview_image(img: Image) {
    unsafe {
        CURRENT_IMAGE = Some(img);
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
                crate::ml::model::gen_model(lock.unwrap(), ui_handle);
            }
        });
    }
}
