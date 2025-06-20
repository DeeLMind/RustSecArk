use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    process::Command,
};
use sha2::{Sha256, Digest};
use curl::easy::Easy;

fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |v: &str| v.split('.').map(|s| s.parse::<u32>().unwrap_or(0)).collect::<Vec<_>>();
    parse(a).cmp(&parse(b))
}

fn get_info(url: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    easy.url(url)?;
    // Enable verbose mode for debugging
    easy.verbose(true)?;
    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        match transfer.perform() {
            Ok(()) => (),
            Err(e) => {
                eprintln!("❌ 无法获取版本信息 (URL: {}): {}", url, e);
                return Err(format!("获取版本信息失败: {}", e).into());
            }
        }
    }
    let text = String::from_utf8(data).map_err(|e| format!("无法解析版本信息: {}", e))?;
    let version = extract_value(&text, "version=").ok_or("版本号未找到")?;
    let sha256 = extract_value(&text, "sha256=").ok_or("SHA256 未找到")?;
    Ok((version, sha256))
}

fn extract_value(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        if let Some(stripped) = line.strip_prefix(key) {
            return Some(stripped.trim().to_string());
        }
    }
    None
}

pub fn update(url:String,info:String,local_version:String,location:String) -> bool {
    // 版本对比是否要下载
    let (version, expected_sha) = get_info(info.as_str()).unwrap();
    if compare_versions(&version, local_version.as_str()) != std::cmp::Ordering::Greater {
        // 无需更新
        return false;
    }

    // 打开文件用于写入
    let mut file = match File::create(location) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("创建临时文件失败: {}", e);
            return false;
        }
    };

    // 初始化 curl easy 对象
    let mut easy = Easy::new();
    if let Err(e) = easy.url(&url) {
        eprintln!("设置 URL 失败: {}", e);
        return false;
    }

    // 连接成功时写入文件
    if let Err(e) = easy.write_function(move |data| {
        if let Err(e) = file.write_all(data) {
            eprintln!("写入文件失败: {}", e);
            return 0; // 触发 curl 中断
        }
        Ok(data.len())
    }) {
        eprintln!("设置写入函数失败: {}", e);
        return false;
    }

    // 执行请求
    if let Err(e) = easy.perform() {
        eprintln!("下载失败: {}", e);
        return false;
    }

    // 检查返回状态码
    let code = easy.response_code().unwrap_or(0);
    if code != 200 {
        eprintln!("HTTP 状态码错误: {}", code);
        let _ = fs::remove_file(location);
        return false;
    }

    // 替换原始文件
    if let Err(e) = fs::rename(location, &location) {
        eprintln!("替换目标文件失败: {}", e);
        return false;
    }

    println!("更新成功，文件已保存至 {}", location);
    true
    // 下载更新文件到指定目录
}

fn main() {
    let url = "https://tspacey.com/TSPACEYRES.exe";
    let verion = "https://tspacey.com/version.txt";
    let sha256 = "https://tspacey.com/sha256.txt";


}