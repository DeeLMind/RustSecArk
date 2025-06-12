use std::any::type_name;

enum Options<T>{
    Some(T),
    None,
}

fn print_type_of<T>(value: &T){
    println!("{}", type_name::<T>());
}

fn process(opt: Option<i32>) {
    if let Some(value) = opt {
        println!("处理值: {}", value);
    } else {
        println!("无值可处理");
    }
}

fn process1(opt: Option<i32>) {
    match opt {
        Some(value) => println!("处理值: {}", value),
        None => println!("无值可处理"),
    }
}

fn main(){
    let name = Some("AAA");
    print!("{:?}",name);
    print_type_of(&name);

    if let Some(name) = name {
        print!("{:?}",name);
    }else {
        print!("None");
    }

    let mut vec = vec![1, 2, 3];

    while let Some(x) = vec.pop() {
        println!("取出 {}", x);
    }

    let config = Some(Some("127.0.0.1"));

    if let Some(Some(ip)) = config {
        println!("IP 地址是 {}", ip);
    }

    let maybe = Some("data");

    let Some(data) = maybe else {
        println!("没有数据");
        return;
    };

    println!("数据是: {}", data);
}
