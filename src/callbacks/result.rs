use crate::appdata::AppState;
use crate::{AppWindow, ResultsGlobal};
use slint::ComponentHandle;
use std::sync::{Arc, Mutex};

pub fn result_callbacks(data: &mut Arc<Mutex<AppState>>, ui: &AppWindow) {
    let global = ui.global::<ResultsGlobal>();
    {
        let data = data.clone();
        let ui_handle = ui.as_weak();
        global.on_gen_beeswarm_img(move |theme| unimplemented!());
    }
}
