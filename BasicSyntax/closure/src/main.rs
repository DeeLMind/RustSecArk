
fn closurefn(){
    let x = 10;
    let add_x = move |y| x + y;
    println!("{}", add_x(5));
}

fn closurefnmut(){
    let name = String::from("Rust");

    // Fn - 只读引用
    let say_hello = || println!("Hello, {}!", name);
    say_hello();
    println!("Hello, {}", name);

    // FnOnce - 移动
    let name = String::from("Move");
    let consume = move || println!("Consumed: {}", name);
    consume(); // name 被移动走，之后不能再使用
    // println!("{}", name);
}

fn closurefnonce(){
    fn apply<F>(f: F)
    where
        F: FnOnce(), // 接收一个实现 FnOnce 的闭包
    {
        f();
    }

    fn create_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }

    let add_one = create_closure();
    println!("{}", add_one(5));
}

fn main() {
    closurefn();
    closurefnmut();
    closurefnonce();
}