// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;

mod chorme_v;
mod config;
mod globalstate;
mod install;
mod logger;
mod pystart;
mod utils;
mod windows;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            pystart::execute_python_script,
            pystart::init_python_path,
            chorme_v::get_chrome_version_command,
            chorme_v::download_chromedriver,
            config::update_json_command,
            config::read_json_command,
            config::get_os_info,
            windows::app_ready,
            install::delayed_restart
        ])
        .setup(|app: &mut tauri::App| {
            let app_config = app.config();

            // 保存 app 为全局变量
            globalstate::APP_HANDLE
                .set(app.handle().clone())
                .expect("Failed to set app handle");

            // 注册日志监听
            logger::configure_logging(app.handle().clone());

            if install::is_first_run(&app_config) {
                info!("首次执行安装");

                // 初始化 settings
                config::init_settings(&app.handle());

                log::info!("detection_environment,{}", config::detection_environment());

                // 安装标识并重启
                install::create_installation_flag(&app_config)
                    .expect("failed to create installation flag");
            } else {
                info!("应用启动");
                // config::init_settings(&app.handle());
                let _ = pystart::activate_python_venv();
            }

            Ok(())
        })
        .on_window_event(|_event| {
            // 窗口事件发生时的操作
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
