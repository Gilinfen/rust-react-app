use std::process::Command;
use log::info;
// use log::{info, warn, error, debug, trace};
use crate::utils::find_command_path;
use crate::config::read_json;

// 执行 python
#[tauri::command]
pub fn execute_python_script(handle: tauri::AppHandle,) -> Result<String, String> {
    match read_json() {
        Ok(data) => {
            info!("Read data: {:?}", data);
        },
        Err(e) => {
            info!("Error reading JSON file: {}", e);
        }
    };

    // 获取 Python 路径
    let python_path = "";
    
    // 执行文件路径
    let python_path_cmd = "../pythonrc/main.pyc";
    let executable_path = if cfg!(debug_assertions) {
        // 开发路径
        python_path_cmd.to_string()
    } else {
        // 生产路径
        handle.path_resolver()
            .resolve_resource(python_path_cmd)
            .expect("failed to resolve resource")
            .to_str().unwrap().to_string()
    };

    info!("python_path: -- {}",python_path);
    info!("executable_path: -- {}",executable_path);

    // 使用获取到的 Python 路径执行命令
    let output = match Command::new(python_path)
        .arg(executable_path)
        .output() {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to execute Python script: {}", e)),
    };

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        info!("SUCCESS: {}",output_str);
        Ok("SUCCESS".to_string())
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr).to_string();
        info!("Python script execution failed: {}", error_str);
        Err(format!("Python script execution failed: {}", error_str))
    }
}
