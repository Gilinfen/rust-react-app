use once_cell::sync::OnceCell;
use std::{path::PathBuf, sync::Mutex};
use tauri::api::path::resource_dir;
use tauri::{AppHandle, Manager};

// 定义一个结构体来封装全局状态
#[derive(Debug)]
pub struct GlobalState {
    pub app_handle: AppHandle,
    /// 应用资源路径
    pub resource_dir: Mutex<Option<PathBuf>>,
}

// 使用 OnceCell 初始化全局状态
pub static GLOBAL_STATE: OnceCell<GlobalState> = OnceCell::new();

impl GlobalState {
    pub fn get_app_handle() -> Option<AppHandle> {
        GLOBAL_STATE.get().map(|state| state.app_handle.clone())
    }

    pub fn get_resource_dir() -> Option<PathBuf> {
        GLOBAL_STATE
            .get()
            .and_then(|state| state.resource_dir.lock().unwrap().clone())
    }

    // 设置全局状态的函数
    pub fn set_global_state(app: &tauri::App) {
        let package_info: tauri::PackageInfo = app.package_info().clone();
        let env: tauri::Env = app.env();

        // 获取资源目录路径
        let resource_dir = resource_dir(&package_info, &env);

        let global_state = GlobalState {
            app_handle: app.handle(),
            resource_dir: Mutex::new(resource_dir),
        };
        GLOBAL_STATE
            .set(global_state)
            .expect("Failed to set global state");
    }
}
