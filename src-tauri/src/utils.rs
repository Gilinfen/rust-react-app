use std::process::Command;
use std::str;

// 通用的函数，允许传入不同的命令
pub fn find_command_path(command_name: &str) -> Result<String, String> {
    let command = if cfg!(target_os = "windows") {
        "where"
    } else {
        "which"
    };

    run_command(command, command_name)
}

pub fn run_command(command: &str, arg: &str) -> Result<String, String> {
    let output = Command::new(command)
        .arg(arg)
        .output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let path_str = str::from_utf8(&o.stdout)
                    .unwrap_or("")
                    .trim()
                    .to_string();
                Ok(path_str)
            } else {
                let err_str = str::from_utf8(&o.stderr)
                    .unwrap_or("Unknown error");
                Err(err_str.to_string())
            }
        },
        Err(e) => Err(e.to_string()),
    }
}