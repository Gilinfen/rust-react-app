use crate::globalstate::APP_HANDLE;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{self, File},
    io::{self, Read},
    process::Command,
    str,
};

/// 路径转换
pub fn resolve_resource_path(resource_path: &str) -> String {
    // let app_handle = APP_HANDLE.get().expect("AppHandle not set");
    let app_handle: &tauri::AppHandle = APP_HANDLE.get().expect("全局 Tauri App 访问失败");
    if cfg!(debug_assertions) {
        // 开发路径
        resource_path.to_string()
    } else {
        // 生产路径
        app_handle
            .path_resolver()
            .resolve_resource(resource_path)
            .expect("failed to resolve resource")
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// 读取 json
pub fn read_json<T: DeserializeOwned>(file_name: &str) -> io::Result<T> {
    let file_path = resolve_resource_path(file_name);
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: T = serde_json::from_str(&contents)?;
    Ok(data)
}

/// 修改 json
pub fn update_json<T: Serialize>(data: &T, file_name: &str) -> io::Result<()> {
    let file_path = resolve_resource_path(file_name);
    let contents = serde_json::to_string(data)?;
    fs::write(file_path, contents)?;
    Ok(())
}

/// 通用的函数，允许传入不同的命令
pub fn find_command_path(command_name: &str) -> Result<String, String> {
    let command = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };

    run_command(command, command_name)
}

pub fn run_command(command: &str, arg: &str) -> Result<String, String> {
    let output = Command::new(command).arg(arg).output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let path_str = str::from_utf8(&o.stdout).unwrap_or("").trim().to_string();
                Ok(path_str)
            } else {
                let err_str = str::from_utf8(&o.stderr).unwrap_or("Unknown error");
                Err(err_str.to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
