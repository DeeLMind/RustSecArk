use std::env;


fn main() {
    let a = std::env::current_exe().ok().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(||"0.0.0.0".into());
    println!("{:?}", a);
}