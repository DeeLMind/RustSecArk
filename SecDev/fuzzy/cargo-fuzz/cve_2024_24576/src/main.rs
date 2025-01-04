use std::io;
use std::process::{Command, Output};

fn execute_command(input: &str) -> Result<Output, std::io::Error> {
    Command::new("./test.bat")
        .arg(input)
        .output()
}

fn main() {
    println!("Input Your Payload:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read from stdin");

    match execute_command(input.trim()) {
        Ok(output) => println!("Output:\n{}", String::from_utf8_lossy(&output.stdout)),
        Err(e) => eprintln!("Failed to execute command: {}", e),
    }
}