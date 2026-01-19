use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(5));

    // 多线程读
    let r1 = Arc::clone(&data);
    let reader = thread::spawn(move || {
        let val = r1.read().unwrap();
        println!("Read: {}", *val);
    });

    // 写
    let w1 = Arc::clone(&data);
    let writer = thread::spawn(move || {
        let mut val = w1.write().unwrap();
        *val += 1;
        println!("Write done");
    });

    reader.join().unwrap();
    writer.join().unwrap();
}
