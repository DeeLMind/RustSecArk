// anyhow 是 Rust 中非常流行的 错误处理框架，专注于简化应用层的错误处理，特别适合快速开发或原型阶段的项目。
// 
// 它提供了一个统一的 Error 类型，可以存储任意具体错误，实现 ? 运算符链式传播，同时保留完整的错误上下文（包括调用栈、错误原因等）。

use anyhow::{Result, anyhow};

fn might_fail(flag: bool) -> Result<String> {
    if flag {
        Ok("成功了".to_string())
    } else {
        Err(anyhow!("发生了错误: flag 为 false"))
    }
}

fn main() -> Result<()> {
    let message = might_fail(false)?;
    println!("{}", message);
    Ok(())
}
