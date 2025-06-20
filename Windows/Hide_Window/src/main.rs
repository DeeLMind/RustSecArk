use std::process::{Command, exit};
use std::path::Path;

fn main() {
    let bat_path = Path::new("example.bat");
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    const DETACHED_PROCESS: u32 = 0x00000008;

    let mut cmd = Command::new(bat_path.to_str().unwrap());
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);

    match cmd.spawn() {
        Ok(_) => {
            println!("成功启动 .bat，本进程即将退出");
            exit(0);
        }
        Err(e) => {
            eprintln!("启动 .bat 失败: {}", e);
        }
    }
}