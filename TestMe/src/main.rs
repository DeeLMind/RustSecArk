fn main() {
    let line = "我和你123";
    let id_index = 5; // 假设从最后一个字符开始
    let mut offset = 0;

    // 按字符计数
    println!("字符数: {}", line.chars().count()); // 输出：字符数: 5

    // 查找空格的循环
    let r = line.chars().nth(0);
    println!("{:?}",r);

    let name = line
        .get(..4)
        .unwrap_or("")
        .trim()
        .to_string();

    println!("{:?}",name);
}