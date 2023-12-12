// src/json_store.rs
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use tauri::command;
use log::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    // 定义您的数据结构
   pub python_path: String
}

fn get_file_path() -> String {
    if cfg!(debug_assertions) {
        // 开发环境的路径
        "/Users/Fox/Code/Rust/rust-react/src-tauri/settings.json".into()
    } else {
        // 生产环境的路径
        "settings.json".into()
    }
}

pub fn read_json() -> io::Result<Data> {
    let file_path = get_file_path();
    info!("file_path {}",file_path);
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: Data = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn update_json(data: &Data) -> io::Result<()> {
    let file_path = get_file_path();
    let contents = serde_json::to_string(data)?;
    fs::write(file_path, contents)?;
    Ok(())
}

#[command]
pub fn read_json_command() -> Result<Data, String> {
    read_json().map_err(|e| e.to_string())
}

#[command]
pub fn update_json_command(data: Data) -> Result<(), String> {
    update_json(&data).map_err(|e| e.to_string())
}