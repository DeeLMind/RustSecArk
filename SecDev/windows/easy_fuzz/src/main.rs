use std::process::{Command, Stdio};
use std::io::Write;
use rand::Rng;
use std::{thread, time};

fn generate_input() -> Vec<u8> {
    let len = rand::rng().random_range(8..=32);
    let mut data = vec![0u8; len];
    for byte in &mut data {
        *byte = rand::rng().random_range(32..=127); // 可打印字符
    }
    data
}

fn run_test(input: &[u8]) -> bool {
    let mut child = Command::new("./BufferOverflow.exe")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start target program");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(input).expect("Failed to write to stdin");
    }

    let result = child.wait();
    match result {
        Ok(status) => !status.success(), // 非正常退出表示可能崩溃
        Err(_) => true,
    }
}

fn main() {
    for i in 0.. {
        let input = generate_input();
        if run_test(&input) {
            println!("🚨 Crash detected with input {}: {:?}", i, String::from_utf8_lossy(&input));
            break;
        }

        if i % 100 == 0 {
            println!("Fuzzed {} cases...", i);
        }

        thread::sleep(time::Duration::from_millis(10)); // 降低 CPU 使用
    }
}
