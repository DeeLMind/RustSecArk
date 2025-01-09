#![feature(windows_process_extensions_show_window)]

use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt;

fn main() {
    // Execute 'winget list' command
    let output = Command::new("winget")
        .show_window(1)
        .arg("list")
        .stdout(Stdio::piped()) // Capture stdout
        .stderr(Stdio::piped()) // Capture stderr
        .output()
        .expect("Failed to execute command");

    // Check if the command was successful
    if output.status.success() {
        // Convert the captured stdout to a string and print it
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("winget list output:\n{}", stdout);
    } else {
        // If command failed, print the error
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("winget list failed with error:\n{}", stderr);
    }
}