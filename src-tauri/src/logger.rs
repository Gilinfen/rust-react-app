use log::{self, Log, Record, Metadata, LevelFilter};
use log::Level;
use tauri::AppHandle;
use tauri::Manager;

pub struct TauriLogger {
    pub app_handle: AppHandle,
}

impl Log for TauriLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args()); // 打印到控制台
            self.app_handle.emit_all("log-message", format!("{}", record.args())).unwrap(); // 发送到前端
        }
    }

    fn flush(&self) {}
}

pub fn configure_logging(app_handle: AppHandle) {
    let logger = TauriLogger { app_handle };
    log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();
}
