use crate::{
    config::{read_json_command, update_json_command},
    utils::run_command,
};
use log::info;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{
    env::temp_dir,
    fs::{self, File},
    io::copy,
    path::{Path, PathBuf},
    process::Command,
    str,
};
use tauri::{AppHandle, Manager};
use tokio::spawn;
use zip::ZipArchive;

#[cfg(target_os = "macos")]
async fn get_chrome_version() -> Result<String, String> {
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

    let chrome_command = format!("{}/Contents/MacOS/Google Chrome", chrome_path);
    run_command(&chrome_command, &["--version"], None).await
}

#[cfg(target_os = "windows")]
fn get_chrome_version() -> Result<String, String> {
    run_command(
        "reg",
        &[
            "query",
            "\"HKEY_CURRENT_USER\\Software\\Google\\Chrome\\BLBeacon\"",
            "/v",
            "version",
        ],
    )
    .or_else(|_| Err("Failed to get Chrome version on Windows".into()))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn get_chrome_version() -> Result<String, String> {
    Err("Unsupported operating system".into())
}

// chorme 版本
#[tauri::command]
pub async fn get_chrome_version_command() -> Result<String, String> {
    let version: Result<String, String> = get_chrome_version().await;
    // match &version {
    //     Ok(v) => info!("Chrome version: {}", v),
    //     Err(e) => info!("Error getting Chrome version: {}", e),
    // }
    version
}

// 获取 chrome/chromedriver url
fn get_file_url(osval: &str, position: &str, files: &str) -> String {
    format!(
        "https://www.googleapis.com/download/storage/v1/b/chromium-browser-snapshots/o/{}%2F{}%2F{}?alt=media",
        osval, position, files
    )
}

async fn download_and_extract(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 获取资源目录路径
    let settings_data = read_json_command()
        .map_err(|e| format!("Error reading settings data: {}", e))
        .unwrap();

    let res_dir: PathBuf = PathBuf::from(&settings_data.res_dir);
    let temp_file = temp_dir().join("temp.zip");

    info!("res_dir --- {:?}", res_dir);

    // 下载文件
    let response = reqwest::get(url).await?;
    let mut file = File::create(&temp_file)?;
    copy(&mut response.bytes().await?.as_ref(), &mut file)?;

    // 解压文件
    let mut archive = ZipArchive::new(File::open(&temp_file)?)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = res_dir.join(file.mangled_name());

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?;
        }
    }

    // 删除下载的文件
    fs::remove_file(temp_file)?;

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
    spawn(async move {
        let file_name = Path::new(&params.files)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        // 获取资源目录路径
        let settings_data = read_json_command()
            .map_err(|e| format!("Error reading settings data: {}", e))
            .unwrap();

        let res_dir: &String = &settings_data.res_dir;

        // 构造下载 URL
        let url: String = get_file_url(&params.os, &params.position, &params.files);

        // 异步下载和解压
        match download_and_extract(&url).await {
            Ok(_) => {
                info!(
                    "Download and extraction completed successfully. {}/{}/chromedriver",
                    res_dir,
                    file_name.to_string()
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

                // 添加权限
                // run_command("chmod", &["+x", &settings_data.chromedriver], None);

                // 设置路径
                match read_json_command() {
                    Ok(mut settings_data) => {
                        // 成功获取 settings_data，现在可以修改它
                        let chromedriver =
                            format!("{}/{}/chromedriver", res_dir, file_name.to_string());
                        settings_data.chromedriver = chromedriver;

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
