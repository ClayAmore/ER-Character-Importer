use std::sync::Mutex;

use saves::{open, save, transfer_character, Save1, Save2};
use tauri::Manager;

mod saves;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app_handle| {
            app_handle.manage((Mutex::new(Save1::default()), Mutex::new(Save2::default())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open, save, transfer_character])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
