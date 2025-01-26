use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use uuid::Uuid;
use std::collections::HashMap;
use std::task::Waker;
use std::sync::mpsc::{Sender, channel};

#[derive(Clone)]
pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, Task>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 注册并执行任务
    pub fn register_task<F>(&self, task_fn: F) -> String
    where
        F: FnOnce() + Send + 'static,
    {
        let task_id = Uuid::new_v4().to_string();
        let task = Task::new(Box::new(task_fn));
        
        self.tasks.lock().unwrap().insert(task_id.clone(), task);
        task_fn(); // 执行任务
        
        task_id
    }

    // 直接执行任务
    pub fn execute_task<F>(&self, task_fn: F)
    where
        F: FnOnce() + Send + 'static,
    {
        tokio::spawn(async move {
            task_fn();
        });
    }

    // 定时执行任务
    pub fn schedule_task<F>(&self, task_fn: F, interval: Duration) -> String
    where
        F: Fn() + Send + 'static,
    {
        let task_id = Uuid::new_v4().to_string();
        let task = Task::new(Box::new(task_fn));

        self.tasks.lock().unwrap().insert(task_id.clone(), task);

        tokio::spawn(async move {
            loop {
                task_fn();
                sleep(interval).await;
            }
        });

        task_id
    }

    // 取消任务
    pub fn cancel_task(&self, task_id: &str) -> Option<()> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(task_id);
        Some(())
    }
}

struct Task {
    task_fn: Box<dyn Fn() + Send>,
}

impl Task {
    fn new<F>(task_fn: F) -> Self
    where
        F: Fn() + Send + 'static,
    {
        Task {
            task_fn: Box::new(task_fn),
        }
    }
}

// 下面是模拟运行时调用的示例代码（main函数）：
#[tokio::main]
async fn main() {
    let manager = TaskManager::new();

    // 注册并执行一个简单任务
    let task_id = manager.register_task(|| {
        println!("This is a registered task");
    });

    println!("Registered task with ID: {}", task_id);

    // 定时执行任务
    let task_id2 = manager.schedule_task(|| {
        println!("This task is executed periodically!");
    }, Duration::from_secs(2));

    println!("Scheduled periodic task with ID: {}", task_id2);

    // 取消任务
    tokio::time::sleep(Duration::from_secs(5)).await;
    manager.cancel_task(&task_id2);
    println!("Canceled task with ID: {}", task_id2);
}
