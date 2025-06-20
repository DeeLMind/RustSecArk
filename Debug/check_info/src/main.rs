use std::io::{self, Write};

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    println!("请输入用户名: ");
    io::stdout().flush().unwrap(); // 确保 prompt 立即显示
    io::stdin().read_line(&mut username).expect("读取用户名失败");
    let username = username.trim();

    println!("请输入密码: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).expect("读取密码失败");
    let password = password.trim();

    if check_credentials(username, password) {
        println!("登录成功！");
    } else {
        println!("用户名或密码错误！");
    }
}

// 简单的“算法”校验，比如用户名+密码的哈希长度之和为某个数
fn check_credentials(username: &str, password: &str) -> bool {
    if username == "deelmind" && password == "geekfz" {
        return true;
    }
    false
}