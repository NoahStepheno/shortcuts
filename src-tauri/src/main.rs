// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod fns;
mod tray;
mod modules {
    pub mod config;
    pub mod cache_manager;
}
mod extensions {
    pub mod clipboard;
    pub mod extension;
    pub mod extension_manager;
}

use tauri::Manager;
use crate::extensions::extension_manager::EXTENSION_MANAGER;

fn main() {
    let extension_manager = &*EXTENSION_MANAGER;
    println!("{}", extension_manager.to_string());
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            command::init,
            command::show_menubar_panel,
            command::get_extensions,
            command::set_extensions
        ])
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let app_handle = app.app_handle();

            tray::create(app_handle)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
