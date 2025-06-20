use std::{
    fs::{self, File,write},
    io::{self, Read, Write,BufReader},
    path::PathBuf,
    path::Path,
    env
};
use std::process::{Command,exit, id as current_pid};
use sha2::{Sha256, Digest};
use curl::easy::Easy;
use std::cmp::Ordering;

pub fn compare_versions(a: &str, b: &str) -> Result<Ordering, String> {
    let parse = |v: &str| -> Result<Vec<u32>, String> {
        v.split('.')
            .map(|s| s.parse::<u32>().map_err(|_| format!("Invalid version part: '{}'", s)))
            .collect()
    };

    let mut va = parse(a)?;
    let mut vb = parse(b)?;

    // 补齐位数（如 "1.0" vs "1.0.0"）
    let max_len = va.len().max(vb.len());
    va.resize(max_len, 0);
    vb.resize(max_len, 0);

    Ok(va.cmp(&vb))
}

fn extract_value(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        if let Some(stripped) = line.strip_prefix(key) {
            return Some(stripped.trim().to_string());
        }
    }
    None
}


fn get_info(url: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let mut easy = Easy::new();
    easy.url(url)?;
    // Enable verbose mode for debugging
    // easy.verbose(true)?;
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

pub fn check_sha256(info: String, update_location: String) -> Result<String, Box<dyn std::error::Error>> {
    // 获取远程的 sha256
    let (_, expected_sha256) = get_info(&info)?; // 你自己的实现，返回 (version, sha256)

    // 打开本地文件
    let file = File::open(Path::new(&update_location))?;
    let mut reader = BufReader::new(file);

    // 计算本地文件 SHA256
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    // 获取 hex 编码
    let result_hash = hasher.finalize();
    let actual_sha256 = hex::encode(result_hash);

    // 比较 hash
    if actual_sha256.eq_ignore_ascii_case(&expected_sha256) {
        Ok("SHA256 校验通过".into())
    } else {
        Err(format!(
            "SHA256 校验失败！期望: {}, 实际: {}",
            expected_sha256, actual_sha256
        ).into())
    }
}

use serde::Serialize;

pub fn update(
    remote_uri_info: String,
    download_url: String,
    local_version: String,
    location: String,
) -> Result<bool, String> {
    let (version, _sha256) = get_info(&remote_uri_info)
        .map_err(|e| format!("获取版本信息失败: {}", e))?;

    let cmp_result = compare_versions(&version, &local_version)
        .map_err(|e| format!("版本对比失败: {}", e))?;

    if cmp_result == std::cmp::Ordering::Greater {
        // 有新版本，执行下载
        let path = std::path::Path::new(&location);
        let mut file = std::fs::File::create(&path)
            .map_err(|e| format!("创建文件失败: {}", e))?;

        let mut easy = curl::easy::Easy::new();
        easy.url(&download_url).map_err(|e| e.to_string())?;
        easy.follow_location(true).map_err(|e| e.to_string())?;

        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                file.write_all(data)
                    .map(|_| data.len())
                    .map_err(|_| curl::easy::WriteError::Pause) // or another variant
            }).expect("");
            transfer.perform().expect("");
        }

        Ok(true) // 成功下载并更新
    } else {
        Ok(false) // 无需更新
    }
}


pub fn do_update(update_file: String, run_file: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut bat_path = env::temp_dir();
    bat_path.push("update_replace.bat");

    let pid = std::process::id();

    let bat_content = format!(
        r#"@echo off
:loop
tasklist /fi "PID eq {pid}" | findstr /i "{exe}" >nul
if %ERRORLEVEL%==0 (
    timeout /t 1 /nobreak >nul
    goto loop
)
timeout /t 1
move /Y "{update}" "{target}"
start "" "{target}"
timeout /t 1
del "%~f0"
"#,
        pid = pid,
        exe = run_file.split('\\').last().unwrap_or(""),
        update = update_file.replace("/", "\\"),
        target = run_file.replace("/", "\\")
    );

    write(&bat_path, bat_content)?;
    println!("写入更新脚本：{}", bat_path.display());

    Command::new("cmd")
        .args(&["/C", bat_path.to_str().unwrap()])
        .spawn()?; // 可考虑加 stdout/stderr 重定向

    exit(0); // 终止当前程序，bat 会完成剩下的任务
}


#[tokio::main]
async fn main() {
    let remote_uri_info = "https://tspacey.com/info.txt";
    let remote_uri_exe = "https://tspacey.com/tspacey.res.exe";
    let temp_dir = PathBuf::from("C:\\Users\\DeeLMind\\Downloads\\tspacey.exe");
    let local_version = "2.0.1";
    // let (version, sha256) = get_info(remote_uri_info).unwrap();
    // let is_download = compare_versions(&version, &local_version).unwrap();
    // println!("{:?}", is_download);
    // check_sha256(remote_uri_info.to_string(),"C:\\Users\\DeeLMind\\Downloads\\tspacey.exe".to_string()).unwrap();;
    // update(remote_uri_info.to_string(),remote_uri_exe.to_string(),local_version.to_string(),"C:\\Users\\DeeLMind\\Downloads\\tspacey.exe".to_string()).unwrap();
    do_update("C:\\Users\\DeeLMind\\Downloads\\tspacey.exe".to_string(),"D:\\Rust\\TargetCargo\\release\\bat.exe".to_string()).unwrap()
}