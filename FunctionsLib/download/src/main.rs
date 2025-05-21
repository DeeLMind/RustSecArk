use curl::easy::Easy;
use std::io::{self, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use curl::easy::List;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

// 简单下载文件
fn easy_download() -> Result<(), Box<dyn std::error::Error>>{
    let mut easy = Easy::new();
    easy.url("https://dldir1.qq.com/qqyy/pc/QQPlayerSetup4.6.3.1104.exe")?;
    let mut file = File::create("D:\\GitHub\\QQPlayerSetup4.6.3.1104.exe")?;

    easy.progress(true)?;
    easy.progress_function(|dltotal, dlnow, _, _| {
        println!("Total: {:.2}M", dltotal / 1024.0 / 1024.0);
        println!("Downloaded: {:.2}%", (dlnow / dltotal) * 100.0);
        true
    })?;

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        file.write_all(data).map(|_| data.len()).map_err(|_| curl::easy::WriteError::Pause)
    })?;
    transfer.perform()?;

    println!("Download completed!");
    Ok(())
}

// 断点续传
fn download_resume() -> Result<(), Box<dyn std::error::Error>>{
    let url = "https://dldir1.qq.com/qqyy/pc/QQPlayerSetup4.6.3.1104.exe";
    let output_path = "D:\\GitHub\\QQPlayerSetup4.6.3.1104.exe";
    let path = Path::new(output_path);

    // 检查文件是否已经存在并且部分下载
    let (mut file, start_position) = if path.exists() {
        let file = OpenOptions::new().append(true).open(path)?;
        let metadata = file.metadata()?;
        (file, metadata.len())
    } else {
        (File::create(path)?, 0)
    };

    println!("Resuming download from byte {}", start_position);

    let mut easy = curl::easy::Easy::new();
    easy.url(url)?;

    // 设置Range请求头
    let mut headers = List::new();
    headers.append(&format!("Range: bytes={}-", start_position))?;
    easy.http_headers(headers)?;

    easy.progress(true)?;
    let start_position = start_position as f64;
    easy.progress_function(move |dltotal, dlnow, _, _| {
        let total = dltotal + start_position;
        let downloaded = dlnow + start_position;
        println!("Total: {:.2}M", total / 1024.0 / 1024.0);
        println!("Downloaded: {:.2}%", (downloaded / total) * 100.0);
        true
    })?;

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        file.write_all(data).map(|_| data.len()).map_err(|_| curl::easy::WriteError::Pause)
    })?;
    transfer.perform()?;

    println!("Download completed!");

    Ok(())
}

// 暂停恢复
fn download_stop() -> Result<(), Box<dyn std::error::Error>>{
let url = "https://dldir1.qq.com/qqyy/pc/QQPlayerSetup4.6.3.1104.exe";
    let output_path = "D:\\GitHub\\QQPlayerSetup4.6.3.1104.exe";
    let path = Path::new(output_path);

    // 检查文件是否已经存在并且部分下载
    let (file, mut start_position) = if path.exists() {
        let file = OpenOptions::new().append(true).open(path)?;
        let metadata = file.metadata()?;
        (Arc::new(std::sync::Mutex::new(file)), metadata.len())
    } else {
        (Arc::new(std::sync::Mutex::new(File::create(path)?)), 0)
    };

    println!("Resuming download from byte {}", start_position);

    let paused = Arc::new(AtomicBool::new(false));
    let paused_clone = Arc::clone(&paused);

    // 启动一个线程来处理用户输入（暂停/继续）
    thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "p" => paused_clone.store(true, Ordering::Relaxed),
            "c" => paused_clone.store(false, Ordering::Relaxed),
            "e" => break,
            _ => println!("Unknown command. Use 'pause', 'continue', or 'exit'."),
        }
    });

    while !paused.load(Ordering::Relaxed) || start_position == 0 {
        let mut easy = curl::easy::Easy::new();
        easy.url(url)?;

        // 设置Range请求头
        let mut headers = List::new();
        headers.append(&format!("Range: bytes={}-", start_position))?;
        easy.http_headers(headers)?;

        let paused_clone_for_progress = Arc::clone(&paused);
        easy.progress(true)?;
        easy.progress_function(move |_, _, _, _| {
            !paused_clone_for_progress.load(Ordering::Relaxed)
        })?;

        let mut transfer = easy.transfer();
        let paused_clone_for_write = Arc::clone(&paused);
        let file_clone = Arc::clone(&file);
        transfer.write_function(move |data| {
            let mut file = file_clone.lock().unwrap();
            if !paused_clone_for_write.load(Ordering::Relaxed) {
                start_position += data.len() as u64;
                file.write_all(data).map(|_| data.len()).map_err(|_| curl::easy::WriteError::Pause)
            } else {
                Err(curl::easy::WriteError::Pause)
            }
        })?;

        let result = transfer.perform();
        if result.is_err() && !paused.load(Ordering::Relaxed) {
            return Err(result.err().unwrap().into());
        }

        if paused.load(Ordering::Relaxed) {
            println!("Download paused. Type 'continue' to resume.");
            while paused.load(Ordering::Relaxed) {
                thread::sleep(std::time::Duration::from_millis(100));
            }
            println!("Download resumed.");
        } else {
            break; // 如果下载没有被暂停并且完成，则退出循环
        }
    }

    println!("Download completed!");

    Ok(())
}

fn main()  {
    let _ = download_stop();
}