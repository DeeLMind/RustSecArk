// macro_rules! 名称 {
//     (匹配模式) => {
//         宏展开代码
//     };
//     // 可定义多个分支匹配不同输入
// }

macro_rules! print_twice {
    ($val:expr) => {
        println!("{}", $val);
        println!("{}", $val);
    };
}

macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! test {
    () => {
        println!("no args");
    };
    ($x:expr) => {
        println!("one arg: {}", $x);
    };
    ($x:expr, $y:expr) => {
        println!("two args: {}, {}", $x, $y);
    };
}

fn main() {
    print_twice!("hello");
    let v = my_vec![1, 2, 3];
    println!("{:?}", v);
}