#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod appdata;
pub mod callbacks;
pub mod savefile;
pub mod config;

use crate::appdata::AppState;
use crate::callbacks::handle_callbacks;
use std::error::Error;
use std::sync::{Arc, Mutex};
use spdlog::info;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let mut app_data = Arc::new(Mutex::new(AppState::default()));

    handle_callbacks(&mut app_data, &ui);

    info!("Starting Beeswarmer");
    ui.run()?;

    Ok(())
}
