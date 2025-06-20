use std::arch::asm;

fn main() {
    #![feature(asm)]
    let mut result: i32;
    unsafe {
        asm!("mov {0}, 42", out(reg) result);
    }
    println!("{}", result);
}
