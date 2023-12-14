use std::env;
use std::fs::{self, File};
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use tauri::api::path::app_data_dir;

fn get_installation_flag_path(config: &tauri::Config) -> PathBuf {
    let app_data_dir = app_data_dir(config).expect("failed to get app data dir");
    app_data_dir.join("installed.flag")
}

pub fn is_first_run(config: &tauri::Config) -> bool {
    !get_installation_flag_path(config).exists()
}

pub fn create_installation_flag(config: &tauri::Config) -> Result<(), Error> {
    let flag_path = get_installation_flag_path(config);
    if let Some(parent) = flag_path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(&flag_path)?;
    Ok(())
}

// 重新启动
#[tauri::command]
pub fn delayed_restart() {
    // 重启应用
    let args: Vec<String> = env::args().collect();
    Command::new(&args[0])
        .args(&args[1..])
        .spawn()
        .expect("failed to restart the application");

    // 退出当前实例
    std::process::exit(0);
}
