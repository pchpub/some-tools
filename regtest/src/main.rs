use std::{fs::File, io::{Read, Write}};

use pcre2::bytes::Regex;
fn main() {
    let mut log_file = File::open("test.log").unwrap();
    let mut buvids_file = File::create("buvids.txt").unwrap();
    let mut buvids = Vec::new();
    let mut binary = [0_u8;4096]; //1的话太小了
    let mut single_sentence = String::new();
    loop {
        let binary_len = log_file.read(&mut binary).unwrap();
        if binary_len == 0 {
            break;
        }
        for single_raw_data in binary.into_iter().zip(0..) {
            if single_raw_data.1 > binary_len {
                break;
            }
            println!("char: {}",std::str::from_utf8(&[single_raw_data.0]).unwrap());
            println!("{}",single_raw_data.0);
            if char::from_u32(single_raw_data.0 as u32).unwrap_or_default() == '\n' {
                // println!(r#"\n Found"#);
                match reg_test(&single_sentence) {
                    Some(value) => {
                        // println!("{}",value);
                        if !buvids.contains(&value) {
                            buvids_file.write(value.as_bytes()).unwrap();
                            buvids_file.write(&[10]).unwrap();
                            buvids.push(value);
                        }
                    },
                    None => (),
                }
                single_sentence = String::new();
            }else{
                single_sentence.push(char::from_u32(single_raw_data.0 as u32).unwrap_or_default());
            }
        }
    }
}

fn reg_test(url: &str) -> Option<String> {
    let re = if let Ok(value) = Regex::new(
        r"buvid=([A-Z0-9]*)",
    ) {
        value
    } else {
        return None;
    };
    let caps = if let Ok(value) = re.captures(url.as_bytes()) {
        match value {
            Some(cap) => cap,
            None => return None,
        }
    } else {
        return None;
    };
    // println!("[ERRORURL_REG] {:?}", caps);
    // Some(caps.get(1).unwrap())
    if let Some(value) = caps.get(1) {
        Some(std::str::from_utf8(value.as_bytes()).unwrap().to_string())
    }else{
        None
    }
}