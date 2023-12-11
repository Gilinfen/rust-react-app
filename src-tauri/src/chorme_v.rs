use std::process::Command;
use std::str;

#[cfg(target_os = "macos")]
fn get_chrome_version() -> Result<String, String> {
    let chrome_path_output = Command::new("mdfind")
        .arg("kMDItemCFBundleIdentifier == 'com.google.Chrome'")
        .output()
        .map_err(|e| e.to_string())?;

    if !chrome_path_output.status.success() {
        return Err("Failed to locate Google Chrome on macOS".into());
    }

    let chrome_path = str::from_utf8(&chrome_path_output.stdout)
        .unwrap_or("")
        .lines()
        .next()
        .unwrap_or("");

    let output = Command::new(format!("{}/Contents/MacOS/Google Chrome", chrome_path))
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(str::from_utf8(&output.stdout).unwrap_or("").trim().to_string())
    } else {
        Err("Failed to get Chrome version on macOS".into())
    }
}

#[cfg(target_os = "windows")]
fn get_chrome_version() -> Result<String, String> {
    // 这里的代码可以根据您的具体需求进行调整
    // 示例代码仅供参考
    let output = Command::new("reg")
        .args(["query", r"HKLM\Software\Google\Chrome\BLBeacon", "/v", "version"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(str::from_utf8(&output.stdout).unwrap_or("").trim().to_string())
    } else {
        Err("Failed to get Chrome version on Windows".into())
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn get_chrome_version() -> Result<String, String> {
    Err("Unsupported operating system".into())
}


#[tauri::command]
pub fn get_chrome_version_command() -> Result<String, String> {
    get_chrome_version()
}
