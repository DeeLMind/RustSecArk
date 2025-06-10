///
/// Mutex 是互斥锁（Mutual Exclusion）的缩写，用于在多线程环境下保护共享数据。Rust 的 Mutex<T> 来自 std::sync 模块，用来确保 同时只有一个线程访问数据。
///
///


use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // 多线程安全的共享值

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 获取锁
            *num += 1; // 修改共享数据
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果: {}", *counter.lock().unwrap());
}