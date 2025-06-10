///
/// “原子引用计数”（Atomic Reference Counted）指针Arc，允许多个线程安全地共享数据,使用 clone 共享数据（不是深拷贝）
///
/// Arc<T> 与 Rc<T> 类似，都是“引用计数智能指针”，用于多个所有者共享同一数据。
///
/// 区别是： Arc 是 线程安全的，使用原子操作来增加/减少引用计数；Rc 只能用于单线程。
///
/// Arc 本身是线程安全的，但其内部数据不一定！Arc<T> ≠ Arc<线程安全的 T>需要搭配 Mutex 或 RwLock 来实现“内部可变性”和线程安全

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn arc_1_test() {
    let a = Arc::new(1);
    let b = a.clone();
    println!("a:{}{:p}-b:{}{:p}",a,a,b,b);
    println!("{:p}-{:p}",&a as *const _,(&b) as *const _);
}

fn arc_2_test() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("结果：{}", *data.lock().unwrap()); // 输出 10
}

fn main() {
    arc_1_test();
}
