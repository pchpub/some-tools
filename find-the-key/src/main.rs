
use std::{sync::Arc, thread};

use rand::Rng;

fn main() {
    let all_data = Arc::new(vec![("access_key=6edc850327a3eaab1da31bd9d44b2622&appkey=191c3b6b975af184&build=7110300&buvid=XUBA871B679CACB56A6F9A53C418CA82465B7&c_locale=zh_CN&channel=master&disable_rcmd=0&local_id=XUBA871B679CACB56A6F9A53C418CA82465B7&mobi_app=android_b&platform=android&s_locale=zh_CN&statistics=%7B%22appId%22%3A3%2C%22platform%22%3A3%2C%22version%22%3A%227.11.0%22%2C%22abtest%22%3A%22%22%7D&ts=1675240615","ccafd3007e65cfd509ff3da6ef190a64")]);
    // let num = Arc::new(AtomicUsize::new(5));
    for _i in 1..20 {
        let all_data = all_data.clone();
        // let num = Arc::clone(&num);
        thread::spawn(move || {
            let mut flag: bool;
            loop {
                flag = true;
                let secret = spawn_random_secret(32);
                // let count_num = num.fetch_add(1, Ordering::SeqCst);
                // if count_num % 100000 == 0 {
                //     println!("{count_num}", count_num = count_num);
                // }
                for data in &*all_data {
                    if !test_secret(data.0, &secret, data.1) {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    println!("{secret}", secret = secret);
                    break;
                }
            } 
        });
    }
    thread::sleep(std::time::Duration::from_secs(9999999999999999));
}

fn test_secret(unsigned_url: &str, secret: &str ,true_sign: &str) -> bool {
    // println!("{}",secret);
    let mut sign = crypto::md5::Md5::new();
    crypto::digest::Digest::input_str(&mut sign, &format!("{unsigned_url}{secret}", unsigned_url = unsigned_url, secret = secret));
    let md5_sign = crypto::digest::Digest::result_str(&mut sign);
    md5_sign.as_str() == true_sign
}

fn spawn_random_secret(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let dist = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'];//, 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    let mut secret = String::new();
    for _ in 0..len {
        secret.push(dist[rng.gen_range(0..16)]);
    }
    secret
}

