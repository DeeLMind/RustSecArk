const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 编码函数
pub fn base64_encode(input: &[u8]) -> String {
    let mut encoded = String::new();
    let mut chunks = input.chunks(3);

    for chunk in chunks {
        let bytes = match chunk.len() {
            3 => [chunk[0], chunk[1], chunk[2], 0],
            2 => [chunk[0], chunk[1], 0, 1], // 最后1表示缺1字节
            1 => [chunk[0], 0, 0, 2],        // 最后2表示缺2字节
            _ => continue,
        };

        let triple = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);

        encoded.push(BASE64_TABLE[((triple >> 18) & 0x3F) as usize] as char);
        encoded.push(BASE64_TABLE[((triple >> 12) & 0x3F) as usize] as char);

        if bytes[3] == 2 {
            encoded.push('=');
            encoded.push('=');
        } else if bytes[3] == 1 {
            encoded.push(BASE64_TABLE[((triple >> 6) & 0x3F) as usize] as char);
            encoded.push('=');
        } else {
            encoded.push(BASE64_TABLE[((triple >> 6) & 0x3F) as usize] as char);
            encoded.push(BASE64_TABLE[(triple & 0x3F) as usize] as char);
        }
    }

    encoded
}

/// 解码函数
pub fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let mut input = input.trim_end_matches('=').to_string();
    if input.len() % 4 == 1 {
        return Err("Invalid Base64 length".to_string());
    }

    let mut bytes = Vec::new();
    let mut buffer = 0u32;
    let mut bits_collected = 0;

    for c in input.chars() {
        let val = match BASE64_TABLE.iter().position(|&x| x == c as u8) {
            Some(v) => v as u32,
            None => return Err(format!("Invalid character: {}", c)),
        };

        buffer = (buffer << 6) | val;
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            let byte = ((buffer >> bits_collected) & 0xFF) as u8;
            bytes.push(byte);
        }
    }

    Ok(bytes)
}

fn main() {
    let text = "DeeLMind";
    let encoded = base64_encode(text.as_bytes());
    println!("Base64 Encoded: {}", encoded);

    let decoded = base64_decode(&encoded).unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();
    println!("Decoded back: {}", decoded_str);
}
