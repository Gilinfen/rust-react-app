// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pystart;
mod utils;
mod chorme_v;
mod logger;
mod config;

use pystart::{execute_python_script};
use chorme_v::{get_chrome_version_command,download_chromedriver};
use logger::configure_logging;
use config::{update_json_command, read_json_command,detection_environment,get_os_info,init_settings};
use utils::{APP_HANDLE};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            execute_python_script,
            get_chrome_version_command,
            update_json_command,
            read_json_command,
            get_os_info,
            download_chromedriver
            ])
        .setup(|app| {
            // 设置全局变量
            APP_HANDLE.set(app.handle().clone()).expect("AppHandle set failed");
            // 注册日志监听
            configure_logging(app.handle().clone());

            log::info!("detection_environment,{}",detection_environment());
 
            // 初始化 settings
            init_settings();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
