use std::fs::File;
use std::io::Read;

fn main() {
    let table = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut data = String::new();
    File::open("data.txt")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();

    let mut flag = String::new();
    for each in data.lines() {
        if each.trim().is_empty() {
            continue;
        }
        let cipher = each.trim();

        let num = cipher.matches('=').count();
        if num == 0 {
            continue;
        }

        let cipher = &cipher[..cipher.len() - num];
        let mut sbin = String::new();
        for i in cipher.chars() {
            sbin.push_str(&format!("{:08b}", table.find(i).unwrap()));
        }

        let hidebit = &sbin[sbin.len() - num * 2..];
        flag.push_str(hidebit);
    }

    // 打印二进制数据
    println!("{}", flag);

    // 将二进制数据转换为ASCII文本
    let flag_bytes = convert_binary_to_bytes(flag);
    if let Ok(flag_text) = String::from_utf8(flag_bytes) {
        println!("Flag text: {}", flag_text);
    } else {
        println!("The binary data was not valid UTF-8.");
    }
}

// 助手函数，将二进制字符串转换为字节向量
fn convert_binary_to_bytes(s: String) -> Vec<u8> {
    s.as_bytes()
        .chunks(8)
        .map(|byte_chunk| {
            let byte_str = std::str::from_utf8(byte_chunk).unwrap();
            u8::from_str_radix(byte_str, 2).unwrap()
        })
        .collect()
}
