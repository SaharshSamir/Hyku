#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app_commands;

use crate::app_commands::sys_tray::{init_sys_tray};

fn main() {
    let tray = init_sys_tray();
    tauri::Builder::default()
        .system_tray(tray)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
