use log::info;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{
    fs::{remove_file, File},
    io::copy,
    path::Path,
    process::Command,
    str,
};
use tauri::{api::path::resource_dir, AppHandle, Manager};
use tokio::spawn;

use crate::config::{read_json_command, update_json_command};

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
        Ok(str::from_utf8(&output.stdout)
            .unwrap_or("")
            .trim()
            .to_string())
    } else {
        Err("Failed to get Chrome version on macOS".into())
    }
}

#[cfg(target_os = "windows")]
fn get_chrome_version() -> Result<String, String> {
    // 这里的代码可以根据您的具体需求进行调整
    // 示例代码仅供参考
    let output = Command::new("reg")
        .args([
            "query",
            r"HKLM\Software\Google\Chrome\BLBeacon",
            "/v",
            "version",
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(str::from_utf8(&output.stdout)
            .unwrap_or("")
            .trim()
            .to_string())
    } else {
        Err("Failed to get Chrome version on Windows".into())
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn get_chrome_version() -> Result<String, String> {
    Err("Unsupported operating system".into())
}

// chorme 版本
#[tauri::command]
pub fn get_chrome_version_command() -> Result<String, String> {
    get_chrome_version()
}

// 获取 chrome/chromedriver url
fn get_file_url(osval: &str, position: &str, files: &str) -> String {
    format!(
        "https://www.googleapis.com/download/storage/v1/b/chromium-browser-snapshots/o/{}%2F{}%2F{}?alt=media",
        osval, position, files
    )
}

/// 下载 解压 chromedriver
async fn download_and_extract(
    url: &str,
    target_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // 下载文件
    let response = reqwest::get(url).await?;
    let mut file = File::create("temp.zip")?;
    copy(&mut response.bytes().await?.as_ref(), &mut file)?;

    // 解压文件
    let mut archive = zip::ZipArchive::new(File::open("temp.zip")?)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target_dir.join(file.mangled_name());

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }

    // 删除下载的文件
    remove_file("temp.zip")?;

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct DownloadParams {
    // 定义对象的字段
    os: String,
    files: String,
    position: String,
}

#[derive(Serialize, Deserialize)]
struct DowReData {
    message: String,
    data: u32,
}

/// 下载 chromedriver
#[tauri::command]
pub async fn download_chromedriver(app_handle: AppHandle, params: DownloadParams) {
    // 从 AppHandle 中获取 PackageInfo 和 Env
    let package_info = app_handle.package_info().clone();
    let env = app_handle.env().clone();

    spawn(async move {
        // 获取资源目录路径
        let res_dir: std::path::PathBuf =
            resource_dir(&package_info, &env).expect("Failed to get resource directory");

        // 构造下载 URL
        let url: String = get_file_url(&params.os, &params.position, &params.files);

        // 异步下载和解压
        match download_and_extract(&url, &res_dir).await {
            Ok(_) => {
                info!(
                    "Download and extraction completed successfully. {:?}{:?}",
                    res_dir, package_info
                );

                let data: DowReData = DowReData {
                    message: "Download and extraction completed successfully".to_string(),
                    data: 200,
                };

                let json: String = to_string(&data).expect("Failed to serialize data");

                // 其他成功处理...
                app_handle
                    .emit_all("message-download-chromedriver", Some(json))
                    .expect("failed to emit event");

                // 设置路径
                match read_json_command() {
                    Ok(mut settings_data) => {
                        let stem = Path::new(&params.files)
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("");
                        // 成功获取 settings_data，现在可以修改它
                        settings_data.chromedriver = format!(
                            "{}/{}/{}/chromedriver",
                            res_dir.to_string_lossy(),
                            package_info.name,
                            stem
                        );
                        let _ = update_json_command(settings_data);
                    }
                    Err(e) => {
                        // 处理读取 JSON 数据时的错误
                        info!("Error reading settings data: {}", e);
                    }
                }
            }
            Err(e) => {
                let data: DowReData = DowReData {
                    message: format!("Error downloading: {}", e),
                    data: 500,
                };
                let json = to_string(&data).expect("Failed to serialize data");

                // 错误处理...
                app_handle
                    .emit_all("message-download-chromedriver", Some(json))
                    .expect("failed to emit event");

                log::error!("Error downloading: {}", e)
            }
        }
    });
}
