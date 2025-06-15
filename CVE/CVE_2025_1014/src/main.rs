fn main() {
    // u16::max 是函数  u16::MAX 是数值
    let data = vec![0u8; 70000];

    println!("u16::MAX {}", u16::MAX);

    if data.len() < u16::max as usize {
        println!("✔️ 数据长度校验通过（实际应失败）");
    } else {
        println!("❌ 数据太长，校验失败");
    }
    
    println!("错误转换后函数指针值（usize）: {}", u16::max as usize);
}