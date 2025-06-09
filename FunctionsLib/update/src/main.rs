use clap::Parser;
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};
use anyhow::{Context, Result};

/// 简单的 Windows 自更新工具
///
/// 典型用法：
/// ```bat
/// updater.exe https://example.com/app.exe %TEMP% C:\ProgramFiles\MyApp -n app.exe
/// ```
#[derive(Parser, Debug)]
#[command(author, version, about = "Rust 自更新器", long_about = None)]
struct Args {
    /// 新版本可执行文件的 URL
    url: String,

    /// 下载目录（例如 %TEMP%）
    download_dir: PathBuf,

    /// 安装目录（当前程序所在的位置）
    install_dir: PathBuf,

    /// 可执行文件名称 (默认 app.exe)
    #[arg(short = 'n', long = "name", default_value = "app.exe")]
    file_name: String,
}

pub fn update_tspacey(uri:&str,download_dir:&str,install_dir:&str,file_name:&str) -> Result<()> {
    let args = Args {
        url: uri.to_string(),
        download_dir: PathBuf::from(download_dir),
        install_dir: PathBuf::from(install_dir),
        file_name: file_name.to_string(),
    };

    // 1. 确保下载目录存在
    if !args.download_dir.exists() {
        fs::create_dir_all(&args.download_dir)
            .with_context(|| format!("Cannot create download directory {}", args.download_dir.display()))?;
    }

    // 下载文件的完整路径
    let download_path = args.download_dir.join(&args.file_name);
    // 安装路径
    let install_path = args.install_dir.join(&args.file_name);

    // 如果安装目录不存在，创建它
    if !args.install_dir.exists() {
        fs::create_dir_all(&args.install_dir)
            .with_context(|| format!("Cannot create install directory {}", args.install_dir.display()))?;
    }

    Ok(())
}

fn main() -> Result<()> {


    // 2. 下载文件
    println!("Downloading {} -> {}", args.url, download_path.display());
    let response = reqwest::blocking::get(&args.url)
        .with_context(|| format!("Failed to GET {}", &args.url))?;

    if !response.status().is_success() {
        anyhow::bail!("HTTP request failed: {}", response.status());
    }

    {
        // 保存到临时文件
        let mut file = File::create(&download_path)
            .with_context(|| format!("Cannot create {}", download_path.display()))?;
        let mut content = std::io::Cursor::new(response.bytes()?);
        std::io::copy(&mut content, &mut file)?;
    }
    println!("Download complete.");

    // 3. 生成批处理脚本
    let bat_path = dirs::home_dir()
        .unwrap_or(PathBuf::from("."))
        .join("run_update.bat");
    let mut bat = File::create(&bat_path)
        .with_context(|| format!("Cannot create {}", bat_path.display()))?;

    writeln!(bat, "@echo off")?;
    writeln!(bat, "echo Applying update…")?;
    writeln!(bat, "timeout /t 1 /nobreak > NUL")?;
    writeln!(bat, "move /y \"{}\" \"{}\"", download_path.display(), install_path.display())?;
    writeln!(bat, "start \"\" \"{}\"", install_path.display())?;
    writeln!(bat, "del \"%~f0\"")?; // 删除脚本自身
    bat.flush()?;

    // 4. 运行批处理并退出本进程
    Command::new("cmd")
        .args(&["/C", bat_path.to_str().unwrap()])
        .spawn()
        .context("Failed to start update batch")?;

    println!("Updater launched, exiting current process.");
    Ok(())
}
