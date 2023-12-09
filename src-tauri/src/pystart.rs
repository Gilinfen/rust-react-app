use std::process::Command;
use crate::utils::find_command_path;

// 执行 python
#[tauri::command]
pub fn execute_python_script() -> Result<String, String> {
    let path = match find_command_path("python3") {
        Ok(p) => p,
        Err(e) => return Err(format!("Error finding python path: {}", e)),
    };

    let output = match Command::new(path)
        .arg("../python/app/main.py")
        .output() {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to execute Python script: {}", e)),
    };

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        println!("Python output: {}", output_str);
        Ok("SUCCESS".to_string())
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Python script execution failed: {}", error_str))
    }
}

// 安装依赖
#[tauri::command]
pub fn python_install() -> Result<String, String> {
    let path = match find_command_path("pip3") {
        Ok(p) => p,
        Err(e) => return Err(format!("Error finding python path: {}", e)),
    };

    let output = match Command::new(path)
        .arg("-V")
        .output() {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to execute Python script: {}", e)),
    };

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        println!("Python output: {}", output_str);
        Ok("SUCCESS".to_string())
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Python script execution failed: {}", error_str))
    } 
}