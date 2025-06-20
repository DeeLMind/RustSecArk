// ! 读作 "never" ,可以自动转换成任何类型
// 表示函数永远不会有返回值，甚至不会返回 ()。
// 典型的用途包括：
// 死循环
// panic!() 崩溃
// process::exit() 直接退出程序
// unreachable!() 逻辑不可能到达的代码

fn never() -> ! {
    panic!("This function never returns!");
}

fn unreachable_case() -> ! {
    unreachable!("This code should never be reached!");
}

fn never_loop() -> ! {
    loop {
        println!("Running forever...");
    }
}

fn infinite() {
    loop {}
}

fn infinite_never() -> ! {
    loop {}
}

fn use_never_type(x: bool) -> i32 {
    if x {
        42
    } else {
        infinite_never() // OK，因为它是 `-> !`
    }
}

fn main() {
    never_loop();
}