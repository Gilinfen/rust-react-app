use std::process::Command;
use std::str;

// 获取 python 路径
#[cfg(target_os = "windows")]
pub fn find_python3_path() -> Result<String, String> {
    run_command("where", "python3")
}

#[cfg(not(target_os = "windows"))]
pub fn find_python3_path() -> Result<String, String> {
    run_command("which", "python3")
}

fn run_command(command: &str, arg: &str) -> Result<String, String> {
    let output = Command::new(command)
        .arg(arg)
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let path_str = str::from_utf8(&o.stdout).unwrap_or("").trim().to_string();
                Ok(path_str)
            } else {
                let err_str = str::from_utf8(&o.stderr).unwrap_or("Unknown error");
                Err(err_str.to_string())
            }
        },
        Err(e) => Err(e.to_string()),
    }
}

// 执行 python
#[tauri::command]
pub fn execute_python_script() -> Result<String, String> {
    let path = match find_python3_path() {
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
pub fn python_install() {
    
}