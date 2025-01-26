use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn thread1() -> i32{
    // 创建线程
    let handle = thread::spawn(|| {
        println!("This is a thread!");
        1
    });

    // 等待线程完成，获取返回值
    let r = handle.join().unwrap();
    println!("Main thread!");
    r
}

fn thread2(){
    let counter = Arc::new(Mutex::new(0)); // 共享数据

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 克隆 Arc
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 获取 Mutex 锁
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // 打印最终结果
}

fn thread3(){
    let (tx, rx) = mpsc::channel(); // 创建通道

    let handle = thread::spawn(move || {
        tx.send("Hello from the thread!").unwrap(); // 发送消息
    });

    let message = rx.recv().unwrap(); // 接收消息
    println!("Received: {}", message);

    handle.join().unwrap();
}

fn main() {
    // let r = thread1();
    // println!("r = {}", r);
    // thread2();
    thread3();
}
