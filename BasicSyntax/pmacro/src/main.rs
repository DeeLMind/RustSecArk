use pmacro::Hello;
use pmacro::log;

#[derive(Hello)]
struct User;

#[log]
fn work() {
    println!("doing work");
}

fn main() {
    User::hello();
    work();
}
