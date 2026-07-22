#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod appdata;
pub mod callbacks;
pub mod config;
mod ml;
pub mod savefile;
mod table;

use crate::appdata::AppState;
use crate::callbacks::handle_callbacks;
use crate::config::Config;
use spdlog::{info, warn};
use std::error::Error;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
pub const SLASH: char = '\\';
#[cfg(not(target_os = "windows"))]
pub const SLASH: char = '/';

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    run_app()?
}

fn run_app() -> Result<Result<(), Box<dyn Error>>, Box<dyn Error>> {
    let ui = AppWindow::new()?;
    
    #[cfg(target_os = "linux")]
    if let Err(e) = slint::set_xdg_app_id("beeswarmer") {
        warn!("Failed to set XDG/Wayland App ID: {}", e);
    }

    let mut app_data = Arc::new(Mutex::new(AppState::default()));

    handle_callbacks(&mut app_data, &ui);

    info!("Loading config");
    let conf = Config::load_config();

    {
        let mut guard = app_data.lock().unwrap();
        guard.french_selected = conf.is_french;
        guard.dark_selected = conf.is_dark;
        ui.set_app_data_slint(guard.clone().into());
    }

    conf.apply_config();

    info!("Starting Beeswarmer");
    ui.invoke_sync_theme();
    ui.run()?;
    info!("Closing Beeswarmer");
    ui.invoke_call_csv_save();

    Ok(Ok(()))
}
