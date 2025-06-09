use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Write,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

use curl::easy::{Easy, WriteError};

/// 单个下载任务
pub struct DownloadTask {
    pub paused: Arc<AtomicBool>,
    pub cancelled: Arc<AtomicBool>,
    pub progress: Arc<Mutex<(u64, u64)>>, // 下载进度
}

impl DownloadTask {
    pub fn new() -> Self {
        Self {
            paused: Arc::new(AtomicBool::new(false)),
            cancelled: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(Mutex::new((0, 0))),
        }
    }

    pub fn start(self: Arc<Self>, url: String, output: String) {
        let task = self.clone();
        thread::spawn(move || {
            let path = Path::new(&output);
            let file = Arc::new(Mutex::new(
                OpenOptions::new().create(true).append(true).open(path).unwrap(),
            ));

            let total_size = {
                let mut easy = Easy::new();
                easy.url(&url).unwrap();
                easy.nobody(true).unwrap();
                easy.perform().unwrap();
                easy.content_length_download().unwrap_or(0.0) as u64
            };
            *task.progress.lock().unwrap() = (0, total_size);

            loop {
                if task.cancelled.load(Ordering::Relaxed) {
                    std::fs::remove_file(path).ok();
                    break;
                }

                let start_pos = file.lock().unwrap().metadata().unwrap().len();
                if total_size > 0 && start_pos >= total_size {
                    break;
                }

                let mut easy = Easy::new();
                easy.url(&url).unwrap();
                easy.resume_from(start_pos).unwrap();

                // 设置进度回调
                {
                    let task = task.clone();
                    easy.progress(true).unwrap();
                    easy.progress_function(move |_, d, _, _| {
                        *task.progress.lock().unwrap() = (start_pos + d as u64, total_size);
                        !(task.paused.load(Ordering::Relaxed) || task.cancelled.load(Ordering::Relaxed))
                    })
                    .unwrap();
                }

                // 写入回调
                {
                    let file = file.clone();
                    let task = task.clone();
                    easy.write_function(move |data| {
                        if task.paused.load(Ordering::Relaxed) || task.cancelled.load(Ordering::Relaxed) {
                            return Err(WriteError::Pause);
                        }
                        let mut f = file.lock().unwrap();
                        f.write_all(data).map(|_| data.len()).map_err(|_| WriteError::Pause)
                    })
                    .unwrap();
                }

                let _ = easy.perform();
                thread::sleep(std::time::Duration::from_millis(200));
            }
        });
    }
}

/// 管理多个下载任务
pub struct DownloadManager {
    pub tasks: Mutex<HashMap<String, Arc<DownloadTask>>>, // key 可用作 ID 或文件名
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(HashMap::new()),
        }
    }

    pub fn start_download(&self, id: String, url: String, output: String) {
        let task = Arc::new(DownloadTask::new());
        task.clone().start(url, output);
        self.tasks.lock().unwrap().insert(id, task);
    }

    pub fn pause(&self, id: &str) {
        if let Some(task) = self.tasks.lock().unwrap().get(id) {
            task.paused.store(true, Ordering::Relaxed);
        }
    }

    pub fn resume(&self, id: &str) {
        if let Some(task) = self.tasks.lock().unwrap().get(id) {
            task.paused.store(false, Ordering::Relaxed);
        }
    }

    pub fn cancel(&self, id: &str) {
        if let Some(task) = self.tasks.lock().unwrap().get(id) {
            task.cancelled.store(true, Ordering::Relaxed);
        }
    }

    pub fn get_progress(&self, id: &str) -> Option<(u64, u64)> {
        self.tasks
            .lock()
            .unwrap()
            .get(id)
            .map(|task| *task.progress.lock().unwrap())
    }
}


fn main() {
    let manager = Arc::new(DownloadManager::new());

    // 启动多个下载任务
    manager.start_download(
        "wechat".to_string(),
        "https://dldir1v6.qq.com/weixin/Universal/Windows/WeChatWin.exe".to_string(),
        "D:\\GitHub\\WeChatWin.exe".to_string(),
    );

    manager.start_download(
        "qq".to_string(),
        "https://dldir1v6.qq.com/weixin/Universal/Windows/WeChatWin.exe".to_string(),
        "D:\\GitHub\\QQ.exe".to_string(),
    );

    // 控制某个任务
    manager.pause("wechat");
    println!("Paused WeChat download.");

    thread::sleep(std::time::Duration::from_secs(1));
    manager.resume("wechat");
    println!("Resumed WeChat download.");

    thread::sleep(std::time::Duration::from_secs(1));
    manager.cancel("qq");
    println!("Cancelled QQ download.");
}
