use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use std::{thread, time};

fn main() -> io::Result<()> {
    // 创建一个命令行进程来执行 ping 命令
    let mut child = Command::new("ping")
        .arg("google.com")  // 目标主机地址
        .arg("-c")           // 指定 ping 次数，-c 1 代表每次只发送 1 个 ping 请求
        .arg("1")
        .stdout(Stdio::piped())  // 捕获标准输出
        .spawn()?;  // 启动子进程

    // 获取子进程的标准输出
    let stdout = child.stdout.take().expect("Failed to capture stdout");

    // 创建一个 BufReader 来逐行读取标准输出
    let reader = io::BufReader::new(stdout);

    // 设置定时器（1 秒钟）
    let one_second = time::Duration::from_secs(1);

    loop {
        // 逐行读取并打印
        match reader.lines().next() {
            Some(Ok(output)) => {
                println!("{}", output);  // 打印每一行
            }
            Some(Err(e)) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
            None => {
                // 如果没有更多输出，等待 1 秒钟后继续读取
                thread::sleep(one_second);
            }
        }

        // 如果需要，也可以判断子进程是否结束
        if let Ok(status) = child.try_wait() {
            if status.is_some() {
                break;
            }
        }
    }

    Ok(())
}
