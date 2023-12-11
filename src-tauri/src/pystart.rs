use std::process::Command;
// use crate::utils::find_command_path;

// 执行 python
#[tauri::command]
pub fn execute_python_script(handle: tauri::AppHandle) -> Result<String, String> {
    let python_path = "../python/dist/main";
    let executable_path = if cfg!(debug_assertions) {
        // 开发路径
        python_path.to_string()
    } else {
        // 生产路径
        handle.path_resolver()
            .resolve_resource(python_path)
            .expect("failed to resolve resource")
            .to_str().unwrap().to_string()
    };
    
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