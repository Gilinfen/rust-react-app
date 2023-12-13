// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod chorme_v;
mod config;
mod logger;
mod pystart;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            pystart::execute_python_script,
            chorme_v::get_chrome_version_command,
            chorme_v::download_chromedriver,
            config::update_json_command,
            config::read_json_command,
            config::get_os_info,
        ])
        .setup(|app: &mut tauri::App| {
            // 设置全局变量
            utils::APP_HANDLE
                .set(app.handle().clone())
                .expect("AppHandle set failed");
            // 注册日志监听
            logger::configure_logging(app.handle().clone());

            log::info!("detection_environment,{}", config::detection_environment());

            // 初始化 settings
            config::init_settings();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
