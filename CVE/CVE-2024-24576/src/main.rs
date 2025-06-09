use std::io;
use std::process::{Command, Stdio};

fn main() {
    println!("Input Your Payload:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // 去掉输入末尾的换行符等空白
    let trimmed_input = input.trim();

    // 第一步：创建命令
    let mut cmd = Command::new("./test.bat");
    cmd.arg(trimmed_input);

    // 如果你想观察构建好的命令，可以打印参数：
    println!("About to run command: ./test.bat {}", trimmed_input);

    // 第二步：执行命令
    let output = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // 第三步：处理输出
    println!("--- STDOUT ---");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("--- STDERR ---");
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
