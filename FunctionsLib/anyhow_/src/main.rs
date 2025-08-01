use anyhow::{anyhow, bail, Context, Result};
use std::fs::File;
use std::io::Read;
use std::net::IpAddr;

fn divide(a: i32, b: i32) -> Result<i32> {
    if b == 0 {
        Err(anyhow!("除数不能为 0")) // 任意错误信息
    } else {
        Ok(a / b)
    }
}

fn read_file(path: &str) -> Result<String> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?; // 自动转为 anyhow::Error
    Ok(content)
}

fn read_with_context(path: &str) -> Result<String> {
    let mut content = String::new();
    File::open(path)
        .with_context(|| format!("无法打开文件: {}", path))?
        .read_to_string(&mut content)
        .context("读取文件内容失败")?;
    Ok(content)
}

fn parse_ip(ip_str: &str) -> Result<IpAddr> {
    if ip_str.is_empty() {
        bail!("IP 字符串不能为空"); // 立即返回错误
    }
    let ip: IpAddr = ip_str.parse().context("IP 地址解析失败")?;
    Ok(ip)
}

fn run_parse_demo() {
    match parse_ip("invalid ip") {
        Err(err) => {
            println!("解析失败：{}", err);
            println!("错误链条：");
            for cause in err.chain() {
                println!(" - {}", cause);
            }
        }
        Ok(ip) => println!("IP 是: {}", ip),
    }
}

fn custom_error() -> Result<()> {
    let err = anyhow!("这是一个自定义错误");
    Err(err)
}

fn nested_error() -> Result<()> {
    fn level1() -> Result<()> {
        Err(anyhow!("Level 1 错误"))
    }

    fn level2() -> Result<()> {
        level1().context("Level 2 上下文")?;
        Ok(())
    }

    fn level3() -> Result<()> {
        level2().context("Level 3 上下文")?;
        Ok(())
    }

    level3()
}

fn main() -> Result<()> {
    match divide(10, 0) {
        Ok(val) => println!("结果: {}", val),
        Err(e) => println!("错误: {}", e),
    }
    
    let _ = read_with_context("not_exists.txt");
    
    run_parse_demo();
    
    let _ = custom_error();
    
    if let Err(e) = nested_error() {
        println!("发生错误：{e}");
        println!("错误链：");
        for cause in e.chain() {
            println!(" - {}", cause);
        }
    }

    Ok(())
}