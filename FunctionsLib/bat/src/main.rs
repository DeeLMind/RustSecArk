use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    process::Command,
};
use sha2::{Sha256, Digest};
use curl::easy::Easy;

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

fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |v: &str| v.split('.').map(|s| s.parse::<u32>().unwrap_or(0)).collect::<Vec<_>>();
    parse(a).cmp(&parse(b))
}

async fn launch_updater_bat_async(old_exe: &PathBuf, new_exe: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let bat_path = old_exe.with_file_name("updater.bat");

    let bat_script = format!(
        r#"
@echo off
echo 🔄 等待主程序退出...

:: 尝试终止当前程序
taskkill /IM "{exe_name}" /F >nul 2>&1
if errorlevel 1 (
    echo ⚠️ 无法终止程序，可能已关闭
)

:: 等待确保程序完全退出
:waitloop
tasklist | findstr /I /C:"{exe_name}" >nul
if not errorlevel 1 (
    timeout /t 1 >nul
    goto waitloop
)

echo ♻️ 替换原程序
move /Y "{new_exe}" "{old_exe}"
if errorlevel 1 (
    echo ❌ 文件替换失败
    exit /b 1
)

echo 🚀 启动新程序
start "" "{old_exe}"
if errorlevel 1 (
    echo ❌ 无法启动新程序
    exit /b 1
)

:: 删除批处理文件自身
del "%~f0"
exit
"#,
        exe_name = old_exe.file_name().unwrap().to_string_lossy(),
        old_exe = old_exe.display(),
        new_exe = new_exe.display(),
    );

    tokio::fs::write(&bat_path, bat_script).await?;

    Command::new("cmd")
        .args(["/C", bat_path.to_str().unwrap()])
        .spawn()?;
    Ok(())
}

pub async fn update(
    remote_uri_info: &str,
    remote_uri_exe: &str,
    temp_dir: &PathBuf,
    local_version: &str,
    self_exe_path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let (version, expected_sha) = get_info(remote_uri_info)?;
    if compare_versions(&version, local_version) != std::cmp::Ordering::Greater {
        println!("✅ 当前已是最新版本：{}", local_version);
        return Ok(());
    }

    println!("🔄 有新版本：{} → {}", local_version, version);

    let exe_url = format!("{}/{}.exe", remote_uri_exe.trim_end_matches('/'), version);
    let file_name = exe_url
        .split('/')
        .last()
        .ok_or("无法从 URL 提取文件名")?
        .to_string();
    let temp_file_path = temp_dir.join(&file_name);

    println!("⬇️ 正在下载新版本文件: {} 到 {}...", file_name, temp_file_path.display());

    // 尝试下载，重试 3 次
    let max_retries = 3;
    let mut attempt = 1;
    let mut last_error = None;

    while attempt <= max_retries {
        println!("尝试下载 (第 {}/{} 次)...", attempt, max_retries);
        let mut file = match File::create(&temp_file_path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("❌ 无法创建文件 {}: {}", temp_file_path.display(), e);
                return Err(e.into());
            }
        };

        let mut easy = Easy::new();
        easy.url(&exe_url)?;
        easy.verbose(true)?; // 启用详细日志
        easy.follow_location(true)?; // 自动处理重定向

        let result = {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                file.write_all(data)
                    .map(|_| data.len())
                    .map_err(|e| curl::easy::WriteError::Pause)
            })?;
            transfer.perform()
        };

        match result {
            Ok(()) => {
                println!("✅ 下载成功: {}", file_name);
                break;
            }
            Err(e) => {
                eprintln!("❌ 下载失败 (第 {}/{} 次): {}", attempt, max_retries, e);
                last_error = Some(e);
                attempt += 1;
                if attempt <= max_retries {
                    println!("⏳ 等待 3 秒后重试...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                }
            }
        }
    }

    if attempt > max_retries {
        return Err(format!("❌ 下载失败，经过 {} 次尝试: {:?}", max_retries, last_error).into());
    }

    println!("🔒 正在校验文件完整性...");
    let mut file = File::open(&temp_file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let actual_sha = format!("{:x}", Sha256::digest(&buffer));

    if actual_sha != expected_sha {
        return Err(format!("❌ 文件校验失败，期望 SHA256: {}, 实际: {}", expected_sha, actual_sha).into());
    }

    println!("✅ 校验成功，准备执行更新程序");

    launch_updater_bat_async(self_exe_path, &temp_file_path).await?;

    println!("⏳ 更新程序启动完成，程序即将退出...");
    std::process::exit(0);
}

#[tokio::main]
async fn main() {
    let remote_uri_info = "https://tspacey.com/info.txt";
    let remote_uri_exe = "https://tspacey.com/tspacey.res.exe";
    let temp_dir = PathBuf::from("C:\\Users\\DeeLMind\\Downloads");
    let local_version = "1.0.1";
    let self_exe_path = PathBuf::from("C:\\Users\\DeeLMind\\Downloads\\winget-cli_x64");

    println!("当前版本: {}", local_version);
    println!("按回车键开始更新...");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    if let Err(e) = update(remote_uri_info, remote_uri_exe, &temp_dir, local_version, &self_exe_path).await {
        eprintln!("更新失败: {}", e);
    }
}