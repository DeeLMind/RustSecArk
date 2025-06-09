use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Instant;

use curl::easy::{Easy, WriteError};

/// 基于 curl crate 的可暂停/恢复下载示例。
/// ▶ p  暂停
/// ▶ c  继续
/// ▶ q  取消
fn download_with_pause_resume() -> Result<(), Box<dyn std::error::Error>> {
    // 下载地址与保存路径
    let url = "https://dldir1v6.qq.com/weixin/Universal/Windows/WeChatWin.exe";
    let output_path = "C:\\Users\\DeeLMind\\Downloads\\WeChatWin.exe";
    let path = Path::new(output_path);

    // 以“追加”方式打开/创建文件，后续所有写入都从文件末尾开始
    let file = Arc::new(Mutex::new(
        OpenOptions::new().create(true).append(true).open(path)?,
    ));

    // 尝试通过 HEAD 请求拿到总大小，便于计算百分比
    let total_size = {
        let mut easy = Easy::new();
        easy.url(url)?;
        easy.nobody(true)?;
        easy.perform()?;
        easy.content_length_download().unwrap_or(0.) as u64
    };

    if total_size == 0 {
        eprintln!("⚠️ 无法从服务器获取文件大小，进度将以字节展示");
    } else {
        println!("文件大小: {total_size} bytes");
    }

    // 控制标志
    let paused = Arc::new(AtomicBool::new(false));
    let cancelled = Arc::new(AtomicBool::new(false));

    // 后台线程读取键盘输入
    {
        let paused = paused.clone();
        let cancelled = cancelled.clone();
        thread::spawn(move || loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "p" => paused.store(true, Ordering::Relaxed),
                "c" => paused.store(false, Ordering::Relaxed),
                "q" => {
                    cancelled.store(true, Ordering::Relaxed);
                    paused.store(false, Ordering::Relaxed);
                    break;
                }
                _ => println!("指令未知，用 p 暂停 / c 继续 / q 取消"),
            }
        });
    }

    // 用于降低进度打印频率
    let last_print = Arc::new(Mutex::new(Instant::now()));

    // 主循环：一次循环对应一次有效下载；若被暂停/取消则中断 transfer 并重新进入循环
    loop {
        if cancelled.load(Ordering::Relaxed) {
            println!("下载已取消");
            std::fs::remove_file(path)?;
            return Ok(());
        }

        // 当前已下载字节（文件长度）
        let start_position = file.lock().unwrap().metadata()?.len();
        if total_size > 0 && start_position >= total_size {
            println!("下载完成 ✅");
            return Ok(());
        }

        println!("➡️  从字节 {start_position} 继续下载...");

        let mut easy = Easy::new();
        easy.url(url)?;
        // resume_from 会让 libcurl 自动加入 Range 头
        easy.resume_from(start_position)?;

        // 进度回调
        {
            let paused = paused.clone();
            let cancelled = cancelled.clone();
            let last_print = last_print.clone();
            easy.progress(true)?;
            easy.progress_function(move |dl_total, dl_now, _, _| {
                if dl_total > 0.0 {
                    let now = Instant::now();
                    let mut last = last_print.lock().unwrap();
                    if now.duration_since(*last).as_secs() >= 1 {
                        let downloaded = start_position as f64 + dl_now;
                        if total_size > 0 {
                            let percent = downloaded / total_size as f64 * 100.0;
                            println!("进度 {:.2}% ({:.0}/{})", percent, downloaded, total_size);
                        } else {
                            println!("已下载 {:.0} bytes", downloaded);
                        }
                        *last = now;
                    }
                }
                !(paused.load(Ordering::Relaxed) || cancelled.load(Ordering::Relaxed))
            })?;
        }

        // 数据写入回调
        {
            let paused = paused.clone();
            let cancelled = cancelled.clone();
            let file_clone = file.clone();
            easy.write_function(move |data| {
                if paused.load(Ordering::Relaxed) || cancelled.load(Ordering::Relaxed) {
                    return Err(WriteError::Pause);
                }
                let mut file = file_clone.lock().unwrap();
                file.write_all(data)
                    .map(|_| data.len())
                    .map_err(|_| WriteError::Pause)
            })?;
        }

        // 执行网络传输
        let result = easy.perform();

        // 若因暂停/取消而终止，忽略错误并重新循环
        if paused.load(Ordering::Relaxed) || cancelled.load(Ordering::Relaxed) {
            thread::sleep(std::time::Duration::from_millis(200));
            continue;
        }

        // 其他错误直接返回
        if let Err(e) = result {
            return Err(e.into());
        }
    }
}

fn main() {
    if let Err(e) = download_with_pause_resume() {
        eprintln!("错误: {e}");
    }
}
