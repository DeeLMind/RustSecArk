mod pe;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::pe::dos_header::parse_dos_header;

fn read_pe_file(path: PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let pe_file = Path::new("C:\\Windows\\System32\\appverifUI.dll");

    match read_pe_file(pe_file.to_path_buf()) {
        Ok(buffer) => {
            if buffer.len() < 64 {
                eprintln!("文件太小，不能是合法 PE 文件");
                return;
            }

            let dos = parse_dos_header(&buffer[0..64]);

            if dos.e_magic != 0x5A4D {
                eprintln!("不是有效的 MZ 文件");
                return;
            }

            println!("DOS Header 解析成功:\n{:#?}", dos);
        }
        Err(e) => {
            eprintln!("读取文件失败: {}", e);
        }
    }
}
