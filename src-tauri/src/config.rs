use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use tauri::command;
use crate::utils::{read_json,update_json};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonData {
    // 定义您的数据结构
   pub python_path: String
}

#[command]
pub fn read_json_command() -> Result<JsonData, String> {
    read_json::<JsonData>("./settings.json").map_err(|e| e.to_string())
}

#[command]
pub fn update_json_command(data: JsonData) -> Result<(), String> {
    update_json(&data, "./settings.json").map_err(|e| e.to_string())
}