use crate::globalstate::GlobalState;
use log::{error, info};
use reqwest;
use std::{
    fs::{remove_file, File},
    io::copy,
    path::Path,
    process::Command,
    str,
};

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
fn get_file_url(val: &str, position: &str, files: &str) -> String {
    format!(
        "https://www.googleapis.com/download/storage/v1/b/chromium-browser-snapshots/o/{}%2F{}%2F{}?alt=media",
        val, position, files
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

/// 下载 chromedriver
#[tauri::command]
pub async fn download_chromedriver() {
    // 启动异步任务进行下载，但不等待它完成
    tokio::spawn(async {
        let url: String = get_file_url("Win", "1226644", "chromedriver_win32.zip");
        if let Some(data_dir) = GlobalState::get_resource_dir() {
            // 处理异步下载和解压，但不阻塞主线程
            if let Err(e) = download_and_extract(&url, &data_dir).await {
                error!("Error downloading: {}", e);
            } else {
                info!("App data directory: {:?}", data_dir);
                info!("解压完成")
            }
            // 使用 data_dir ...
        } else {
            error!("Failed to get app data directory.");
        }
    });
}
