use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new(); // 创建空 HashMap

    scores.insert("A", 100);     // 插入键值对
    scores.insert("B", 95);

    println!("{:?}", scores);        // 打印整个 HashMap

    if let Some(score) = scores.get("A") {
        println!("A 分数是：{}", score);
    } else {
        println!("查无此人");
    }

    for (key, value) in &scores {
        println!("{} => {}", key, value);
    }
}
