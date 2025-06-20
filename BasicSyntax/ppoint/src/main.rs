
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let u1 = User { name: "Alice".into(), age: 30 };
    let u2 = User { ..u1 };
    // println!("{:?}",u1);
    println!("{:?}",u2);
}
