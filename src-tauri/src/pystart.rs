use log::info;
use serde::{Deserialize, Serialize};
// use log::{info, warn, error, debug, trace};
use crate::config::{read_json_command, update_json_command};
use crate::utils::{find_command_path, resolve_resource_path, run_command};

/// 激活 python 虚拟环境
pub async fn activate_python_venv() -> Result<String, String> {
    let settings_data =
        read_json_command().map_err(|e| format!("Error reading settings data: {}", e))?;
    let res_dir: &String = &settings_data.res_dir;

    // 激活虚拟环境
    if cfg!(target_os = "windows") {
        // Windows 系统
        run_command("cmd", &["/C", ".\\venv\\Scripts\\activate"], Some(&res_dir)).await
    } else {
        // macOS / Linux 系统
        // 注意：这种方式通常不会按预期工作，因为 `source` 是 shell 内建命令
        run_command(
            "bash",
            &["-c", "source ./venv/bin/activate"],
            Some(&res_dir),
        )
        .await
    }
}

/// 初始化 python
#[tauri::command]
pub async fn init_python_path() -> Result<(), String> {
    let settings_data =
        read_json_command().map_err(|e| format!("Error reading settings data: {}", e))?;
    // let res_dir = resolve_resource_path("../");
    let res_dir: &String = &settings_data.res_dir;

    let python: String = find_command_path("python3")
        .await
        .map_err(|e| format!("Error finding python path: {}", e))?;
    let pip = find_command_path("pip3")
        .await
        .map_err(|e| format!("Error finding pip path: {}", e))?;

    info!("res_dir : {}", res_dir);
    info!("Python path: {}", python);
    info!("Pip path: {}", pip);

    // 创建虚拟环境
    let _ = run_command(&python, &["-m", "venv", "venv"], Some(&res_dir)).await;

    // 激活虚拟环境
    let _ = activate_python_venv();

    // 根据虚拟环境路径构建 Python 和 Pip 的路径
    let venv_python_path: String = format!("{}/venv/bin/python", res_dir);
    let venv_pip_path: String = format!("{}/venv/bin/pip", res_dir);

    info!("venv_python_path: {}", python);
    info!("venv_python_path: {}", pip);

    let mut updated_settings = settings_data;
    updated_settings.python_path = python;
    updated_settings.pip_path = pip;
    updated_settings.venv_python_path = venv_python_path;
    updated_settings.venv_pip_path = venv_pip_path;

    update_json_command(updated_settings)
        .map_err(|e| format!("Error updating settings data: {}", e))
}

/// 执行 python 命令
pub async fn cmd_python_script(
    python_path: &str,
    executable_cmd: &[&str],
) -> Result<String, String> {
    let settings_data =
        read_json_command().map_err(|e: String| format!("Error reading settings data: {}", e))?;
    let res_dir: &String = &settings_data.res_dir;
    info!("res_dir: -- {}", res_dir);
    // 执行文件路径
    info!("python_path: -- {}", python_path);

    info!("executable_path: -- {:?}", executable_cmd);
    run_command(&python_path, &executable_cmd, Some(&res_dir)).await
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

    let python: &String = &settings_data.venv_python_path;
    let pip: &String = &settings_data.venv_pip_path;

    match cmd_type {
        CommandType::Python => {
            let executable_path = resolve_resource_path("../pythonrc/main.pyc");
            cmd_python_script(&python, &[&executable_path]).await
        }
        CommandType::Pip => {
            let executable_path = resolve_resource_path("../pythonrc/requirements.txt");
            cmd_python_script(&pip, &["install", "-r", &executable_path]).await
        }
    }
}
