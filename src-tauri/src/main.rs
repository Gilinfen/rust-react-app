// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pystart;
mod utils;
mod chorme_v;
mod logger;
mod config;

use pystart::execute_python_script;
use chorme_v::get_chrome_version_command;
use logger::configure_logging;
use config::{update_json_command, read_json_command};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            execute_python_script,
            get_chrome_version_command,
            update_json_command,
            read_json_command
            ])
        .setup(|app| {
            // 注册日志监听
            configure_logging(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
