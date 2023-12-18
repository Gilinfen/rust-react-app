use std::process::Command;
use std::str;

#[cfg(target_os = "macos")]
pub fn get_uuid() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("ioreg").args(["-l"]).output()?;

    if output.status.success() {
        let output_str = str::from_utf8(&output.stdout)?;
        for line in output_str.lines() {
            if line.contains("IOPlatformUUID") {
                let parts: Vec<&str> = line.split("=").collect();
                if parts.len() > 1 {
                    return Ok(parts[1].trim().trim_matches('"').to_string());
                }
            }
        }
    }

    Err("Could not find IOPlatformUUID".into())
}

#[cfg(target_os = "windows")]
pub fn get_uuid() -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;

    let output = Command::new("wmic")
        .args(["cpu", "get", "ProcessorId"])
        .output()?;

    if output.status.success() {
        let output_str = String::from_utf8(output.stdout)?;
        for line in output_str.lines() {
            if !line.is_empty() && line != "ProcessorId" {
                return Ok(line.trim().to_string());
            }
        }
    }

    Err("Could not find CPU Serial Number".into())
}
