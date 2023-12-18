use log::info;
use std::env;
use std::fs::{self, File};
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use tauri::api::path::app_data_dir;

fn get_app_data_flag_path(config: &tauri::Config, path: &str) -> PathBuf {
    let app_data_dir = app_data_dir(config).expect("failed to get app data dir");
    app_data_dir.join(path)
}

pub fn is_first_run(config: &tauri::Config) -> bool {
    !get_app_data_flag_path(config, "installed.flag").exists()
}

pub fn create_app_data_flag(config: &tauri::Config, path: &str) -> Result<PathBuf, Error> {
    let flag_path: PathBuf = get_app_data_flag_path(config, path);
    if let Some(parent) = flag_path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(&flag_path)?;
    Ok(flag_path)
}

// 创建激活文件
pub fn ccreate_conf_activate(config: &tauri::Config) {
    let activate_path = get_app_data_flag_path(config, "activate");

    if activate_path.exists() {
        info!("activate_path")
    } else {
        let flag_path: PathBuf = create_app_data_flag(config, "activate").unwrap();
        info!("flag_path： {:?}", flag_path);
    }
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
