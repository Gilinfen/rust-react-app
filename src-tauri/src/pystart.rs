use std::process::Command;
use log::info;
// use log::{info, warn, error, debug, trace};
use crate::utils::{find_command_path,resolve_resource_path};
use crate::config::{read_json_command,update_json_command};

pub fn init_python_path() {
    let result = find_command_path("python3");

    match result {
        Ok(pypath) => {
            // 首先，处理 read_json_command 返回的 Result
            match read_json_command() {
                Ok(mut settings_data) => {
                    // 成功获取 settings_data，现在可以修改它
                    settings_data.python_path = pypath;
                    let _ = update_json_command(settings_data);
                },
                Err(e) => {
                    // 处理读取 JSON 数据时的错误
                    info!("Error reading settings data: {}", e);
                }
            }
        },
        Err(e) => {
            // 处理寻找 python 路径时的错误
            info!("Error finding python path: {}", e);
        }
    }
}

// 执行 python
#[tauri::command]
pub fn execute_python_script() -> Result<String, String> {
    let settings_data = read_json_command();

    // 获取 Python 路径
    let python_path = match settings_data {
        Ok(data) => data.python_path,
        Err(e) => return Err(e),
    };
    
    // 执行文件路径
    let executable_path = resolve_resource_path("../pythonrc/main.pyc");

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
