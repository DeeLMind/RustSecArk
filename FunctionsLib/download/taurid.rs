use std::io::{self, Write};
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use curl::easy::List;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

/// 下载软件
/// 
/// # 参数
/// 
/// * `software_uri` - 软件的URI地址
/// 
/// # 返回值
/// 
/// * `Result<(), Box<dyn std::error::Error>>` - 返回Ok表示下载成功，Err表示下载失败
pub fn download(software_uri:&str,location_dir:&str) -> Result<(), Box<dyn std::error::Error>>{
    // 提取文件名
    let filename = Path::new(software_uri)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Invalid URL or filename")?;

    // 创建输出目录
    let path = Path::new(location_dir).join(filename);

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
        easy.url(software_uri)?;

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