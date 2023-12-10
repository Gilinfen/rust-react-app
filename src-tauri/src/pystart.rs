use std::process::Command;
// use crate::utils::find_command_path;

// 执行 python
#[tauri::command]
pub fn execute_python_script() -> Result<String, String> {
    let executable_path = "../python/dist/main"; // 更新为你的可执行文件的路径

    let output = match Command::new(executable_path)
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