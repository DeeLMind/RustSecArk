use std::thread;

fn main() {
    for i in 0..5 {
        thread::spawn(move || {
            println!("Thread {} is running", i);
        });
        // println!("Main thread is running {}", i);
    }

    for s in vec!["A".to_string(), "B".to_string()] {
        thread::spawn(move || {
            println!("Thread says {}", s);
        });

        // println!("Main says {}", s); // ❌ 会报错，s 被 move 了
    }

    // 等待所有线程执行完，真实项目中你应该保存 handle.join()
    thread::sleep(std::time::Duration::from_millis(50));
}