use log::info;
use serde::{Deserialize, Serialize};
// use log::{info, warn, error, debug, trace};
use crate::config::{read_json_command, update_json_command};
use crate::utils::{find_command_path, resolve_resource_path, run_command};

/// 初始化 python
pub async fn init_python_path() -> Result<(), String> {
    let settings_data =
        read_json_command().map_err(|e| format!("Error reading settings data: {}", e))?;

    let pypath = find_command_path("python3")
        .await
        .map_err(|e| format!("Error finding python path: {}", e))?;
    let pip = find_command_path("pip3")
        .await
        .map_err(|e| format!("Error finding pip path: {}", e))?;

    info!("Python path: {}", pypath);
    info!("Pip path: {}", pip);

    let mut updated_settings = settings_data;
    updated_settings.python_path = pypath;
    updated_settings.pip_path = pip;
    update_json_command(updated_settings)
        .map_err(|e| format!("Error updating settings data: {}", e))
}

/// 定义 python 执行路径
pub async fn cmd_python_script(
    python_path: &str,
    executable_cmd: &[&str],
) -> Result<String, String> {
    let res_dir = resolve_resource_path("../");
    info!("res_dir: -- {}", res_dir);
    // 执行文件路径
    info!("python_path: -- {}", python_path);

    info!("executable_path: -- {:?}", executable_cmd);
    run_command(&python_path, &executable_cmd, Some(&res_dir)).await
    // .or_else(|e| Err(format!("Failed to execute Python script: {}", e)))
}

#[derive(Deserialize, Serialize)]
pub enum CommandType {
    Python,
    Pip,
}

/// 执行 python
#[tauri::command]
pub async fn execute_python_script(cmd_type: CommandType) -> Result<String, String> {
    // 获取 settings json 内容
    let settings_data = read_json_command()?;

    match cmd_type {
        CommandType::Python => {
            let executable_path = resolve_resource_path("../pythonrc/main.pyc");
            cmd_python_script(&settings_data.python_path, &[&executable_path]).await
        }
        CommandType::Pip => {
            let executable_path = resolve_resource_path("../pythonrc/requirements.txt");
            cmd_python_script(
                &settings_data.pip_path,
                &["install", "-r", &executable_path],
            )
            .await
        }
    }
}
