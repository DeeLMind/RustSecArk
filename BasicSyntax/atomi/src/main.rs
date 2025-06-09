use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);

    let handle = thread::spawn(move || {
        // 设置为 true
        flag_clone.store(true, Ordering::Relaxed);
    });

    handle.join().unwrap();

    // 读取状态
    if flag.load(Ordering::Relaxed) {
        println!("Flag is true!");
    } else {
        println!("Flag is false!");
    }
}
