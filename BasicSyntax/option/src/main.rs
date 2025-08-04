fn main() {
    // 1. 创建 Option
    let some_val = Some(42);
    let no_val: Option<i32> = None;

    // 2. unwrap()
    println!("unwrap: {}", some_val.unwrap());
    // println!("unwrap None: {}", no_val.unwrap()); // ❌ panic

    // 3. unwrap_or
    println!("unwrap_or: {}", no_val.unwrap_or(100));

    // 4. unwrap_or_else
    println!("unwrap_or_else: {}", no_val.unwrap_or_else(|| {
        println!("计算默认值");
        200
    }));

    // 5. unwrap_or_default
    let default_val: Option<String> = None;
    println!("unwrap_or_default: {}", default_val.unwrap_or_default());

    // 6. map
    let x = Some(3);
    let y = x.map(|v| v * 10);
    println!("map: {:?}", y);

    // 7. map_or
    let x: Option<i32> = Some(3);
    let y = x.map_or(0, |v| v * 2);
    println!("map_or: {}", y);

    // 8. map_or_else
    let x: Option<i32> = None;
    let y = x.map_or_else(|| 10, |v| v * 2);
    println!("map_or_else: {}", y);

    // 9. is_some / is_none
    println!("is_some: {}", some_val.is_some());
    println!("is_none: {}", no_val.is_none());

    // 10. and
    let a = Some(1);
    let b = Some("ok");
    let c = a.and(b);
    println!("and: {:?}", c); // Some("ok")

    // 11. and_then
    let a = Some(2);
    let b = a.and_then(|x| Some(x * 3));
    println!("and_then: {:?}", b); // Some(6)

    // 12. or
    let a: Option<i32> = None;
    let b = Some(42);
    let c = a.or(b);
    println!("or: {:?}", c); // Some(42)

    // 13. or_else
    let a: Option<i32> = None;
    let b = a.or_else(|| Some(123));
    println!("or_else: {:?}", b);

    // 14. match 模式匹配
    let name = Some("Alice");
    match name {
        Some(n) => println!("match: Hello, {}!", n),
        None => println!("match: Who are you?"),
    }

    // 15. if let
    if let Some(v) = Some(99) {
        println!("if let: Got {}", v);
    }

    // 16. while let
    let mut opt = Some(1);
    while let Some(v) = opt {
        println!("while let: {}", v);
        opt = if v < 3 { Some(v + 1) } else { None };
    }

    // 17. expect
    let user = Some("Bob");
    println!("expect: {}", user.expect("必须有用户名"));

    // 18. filter
    let x = Some(10);
    let filtered = x.filter(|&n| n > 5);
    println!("filter >5: {:?}", filtered);

    let filtered_none = x.filter(|&n| n > 100);
    println!("filter >100: {:?}", filtered_none);

    // 19. flatten
    let nested = Some(Some(999));
    let flat = nested.flatten();
    println!("flatten: {:?}", flat);

    // 20. ok_or / ok_or_else
    let maybe_number = Some(8);
    let res: Result<i32, &str> = maybe_number.ok_or("错误");
    println!("ok_or: {:?}", res);

    let maybe_none: Option<i32> = None;
    let res2 = maybe_none.ok_or_else(|| "默认错误信息");
    println!("ok_or_else: {:?}", res2);

    // 21. into_iter
    let opt = Some("value");
    for v in opt {
        println!("into_iter: {}", v);
    }

    // 22. 实战示例：读取环境变量
    let config = std::env::var("MY_ENV_VAR").ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| {
            println!("使用默认配置");
            "default".to_string()
        });

    println!("环境变量配置: {}", config);

    // 23. 和函数组合使用
    let parsed = Some("123").and_then(|s| s.parse::<i32>().ok());
    println!("字符串转数字: {:?}", parsed);
}