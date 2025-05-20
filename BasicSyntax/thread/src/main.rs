use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::task;

#[derive(Default)]
pub struct TaskManager {
    tasks: Mutex<HashMap<String, tokio::task::JoinHandle<()>>>,
}

impl TaskManager {
    // 启动一个任务
    pub async fn start_task(&self, task_name: String) {
        let mut tasks = self.tasks.lock().await;

        if tasks.contains_key(&task_name) {
            println!("任务 {} 已经在执行中", task_name);
            return; // 如果任务已在执行中，则不再启动
        }

        // 创建一个新的任务
        let handle = tokio::spawn(async move {
            println!("任务 {} 执行中...", task_name);
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            println!("任务 {} 执行完成", task_name);
        });

        tasks.insert(task_name, handle);
    }

    // 停止一个任务
    pub async fn stop_task(&self, task_name: String) {
        let mut tasks = self.tasks.lock().await;
        if let Some(handle) = tasks.remove(&task_name) {
            // 在这里可以尝试取消任务，尽管 tokio 并不直接支持任务的中断
            println!("任务 {} 已取消", task_name);
        }
    }
}

async fn start_task(manager: tauri::State<'_, Arc<TaskManager>>, task_name: String) {
    manager.start_task(task_name).await;
}

async fn stop_task(manager: tauri::State<'_, Arc<TaskManager>>, task_name: String) {
    manager.stop_task(task_name).await;
}

fn main() {
    let task_manager = Arc::new(TaskManager::default());

    // 启动任务
    let task_name = "task1".to_string();
    let manager_clone = task_manager.clone();
    tokio::spawn(async move {
        start_task(manager_clone, task_name).await;
    });

    // 停止任务
    let task_name = "task1".to_string();
    let manager_clone = task_manager.clone();
    tokio::spawn(async move {
        stop_task(manager_clone, task_name).await;
    });
}