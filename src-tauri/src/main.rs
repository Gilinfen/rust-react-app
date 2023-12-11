// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pystart;
mod utils;

use pystart::execute_python_script;
use pystart::python_path;
use pystart::init_python_path;

use log::{Record, Level, Metadata, LevelFilter};
use log::info;
use tauri::AppHandle;
use tauri::Manager;

struct TauriLogger {
    app_handle: AppHandle,
}

// 日志监听发送至前端
impl log::Log for TauriLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args()); // 打印到控制台
            self.app_handle.emit_all("log-message", format!("{}", record.args())).unwrap(); // 发送到前端
        }
    }

    fn flush(&self) {}
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,execute_python_script,python_path])
        .setup(|app| {
            // 初始化 Python 路径
            if let Err(e) = init_python_path() {
                info!("Failed to initialize Python path: {}", e);
                // 这里你可以决定是否要中止应用
            }
            // 注册日志监听
            let logger = TauriLogger { app_handle: app.handle().clone() };
            log::set_boxed_logger(Box::new(logger))
                .map(|()| log::set_max_level(LevelFilter::Info))
                .unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
