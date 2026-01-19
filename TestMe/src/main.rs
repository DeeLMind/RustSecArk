use std::{fs::read, io::Read};

fn main() {
    let mut f = std::fs::File::open("path").unwrap();
    let mut buf = [0u8; 1024];
    f.read(&mut buf);
}
