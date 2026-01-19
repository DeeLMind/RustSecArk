// ============================================================
// Rust match
// 覆盖：
// - 基础 match
// - 字面量 / 范围 / |
// - match 是表达式
// - enum / struct / tuple 解构
// - Option / Result
// - if guard
// - @ 绑定
// - ref / ref mut / & / &mut
// - 忽略模式 _ / ..
// - while let
// - matches! 宏
// - 所有权 / 借用差异
// ============================================================

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // ========================================================
    // 1. 基础 match（字面量 + _）
    // ========================================================
    let x = 3;

    let desc = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };

    println!("basic match: {desc}");

    // ========================================================
    // 2. match 是表达式
    // ========================================================
    let flag = true;

    let v = match flag {
        true => 1,
        false => 0,
    };

    println!("match as expression: {v}");

    // ========================================================
    // 3. 范围 / 多模式 |
    // ========================================================
    let n = 7;

    match n {
        1..=5 => println!("1 to 5"),
        6 | 7 | 8 => println!("6 or 7 or 8"),
        _ => println!("other"),
    }

    // ========================================================
    // 4. tuple 解构
    // ========================================================
    let pair = (0, 10);

    match pair {
        (0, y) => println!("x is zero, y={y}"),
        (x, 0) => println!("y is zero, x={x}"),
        (x, y) => println!("x={x}, y={y}"),
    }

    // ========================================================
    // 5. struct 解构
    // ========================================================
    let p = Point { x: 3, y: 0 };

    match p {
        Point { x, y: 0 } => println!("on x-axis, x={x}"),
        Point { x: 0, y } => println!("on y-axis, y={y}"),
        Point { x, y } => println!("point ({x}, {y})"),
    }

    // ========================================================
    // 6. enum 解构（最重要）
    // ========================================================
    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to {x},{y}"),
        Message::Write(text) => println!("write: {text}"),
        Message::ChangeColor(r, g, b) => println!("color {r},{g},{b}"),
    }

    // ========================================================
    // 7. Option
    // ========================================================
    let opt = Some(42);

    match opt {
        Some(v) => println!("Option has value {v}"),
        None => println!("Option is None"),
    }

    // ========================================================
    // 8. Result
    // ========================================================
    let res: Result<i32, &str> = Ok(100);

    match res {
        Ok(v) => println!("Result Ok {v}"),
        Err(e) => println!("Result Err {e}"),
    }

    // ========================================================
    // 9. 嵌套 match（Option<Result<T,E>>）
    // ========================================================
    let nested: Option<Result<i32, &str>> = Some(Ok(5));

    match nested {
        Some(Ok(v)) => println!("Some Ok {v}"),
        Some(Err(e)) => println!("Some Err {e}"),
        None => println!("None"),
    }

    // ========================================================
    // 10. if guard
    // ========================================================
    let num = 10;

    match num {
        n if n % 2 == 0 => println!("{n} is even"),
        _ => println!("odd"),
    }

    // ========================================================
    // 11. @ 绑定
    // ========================================================
    let value = 8;

    match value {
        n @ 1..=10 => println!("in range 1..10: {n}"),
        n @ 1..=10 if n > 6 => println!("in range 1..10: {n}"),
        _ => println!("out of range"),
    }

    // ========================================================
    // 12. 所有权问题：默认 move
    // ========================================================
    let s = Some(String::from("owned"));

    match s {
        Some(v) => println!("moved value: {v}"),
        None => {}
    }
    // s 在这里已经不可用（被 move）

    // ========================================================
    // 13. 借用 match（推荐写法）
    // ========================================================
    let s2 = Some(String::from("borrowed"));

    match &s2 {
        Some(v) => println!("borrowed value: {v}"),
        None => {}
    }

    println!("s2 still usable: {:?}", s2);

    // ========================================================
    // 14. ref / ref mut（旧但重要）
    // ========================================================
    let mut s3 = Some(String::from("ref"));

    match s3 {
        Some(ref mut v) => v.push_str("_mut"),
        None => {}
    }

    println!("after ref mut: {:?}", s3);

    // ========================================================
    // 15. 忽略字段 ..
    // ========================================================
    let p2 = Point { x: 10, y: 20 };

    match p2 {
        Point { x, .. } => println!("only care x={x}"),
    }

    // ========================================================
    // 16. while let（循环匹配）
    // ========================================================
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("pop {top}");
    }

    // ========================================================
    // 17. matches! 宏
    // ========================================================
    let check = Some(5);

    if matches!(check, Some(v) if v > 3) {
        println!("matches! works");
    }

    // ========================================================
    // 18. & / &mut 解构匹配
    // ========================================================
    let num = 5;

    match &num {
        5 => println!("reference match 5"),
        _ => {}
    }

    let mut num2 = 10;

    match &mut num2 {
        n if *n > 5 => *n += 1,
        _ => {}
    }

    println!("num2 after &mut match: {num2}");

    // ========================================================
    // END
    // ========================================================
}
