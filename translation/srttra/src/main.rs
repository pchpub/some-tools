use std::{io::Write, fs::{File, read_to_string}};
use curl::easy::Easy;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TraConfig {
    line_type: String,
    appid: String,
    appkey: String,
    from_lang: String,
    to_lang: String,
}

pub fn gettra(text: &str, config: &TraConfig) -> Option<String> {
    let mut rng = rand::thread_rng();
    let salt : u32 = rng.gen_range(32768..65536);
    let sign = md5::compute(format!("{}{text}{salt}{}",config.appid,config.appkey));
    let query_vec = vec![
        ("appid",config.appid.clone()),
        ("q",text.to_string()),
        ("from",config.from_lang.clone()),
        ("to",config.to_lang.clone()),
        ("salt",salt.to_string()),
        ("sign",format!("{:x}",sign)),
    ];
    let request_body_str = format!("{}",qstring::QString::new(query_vec));
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&format!("http://api.fanyi.baidu.com/api/trans/vip/translate?{request_body_str}")).unwrap();
    handle.post(false).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        match transfer.perform() {
            Ok(()) => (),
            _error => {
                return None;
            }
        }
    }

    let getwebpage_string: String = match String::from_utf8(data){
        Ok(value) => value,
        Err(_) => return None,
    };
    //println!("{getwebpage_string}");
    let getwebpage_json: serde_json::Value = serde_json::from_str(&getwebpage_string).unwrap();
    match getwebpage_json["trans_result"][0]["dst"].as_str() {
        Some(value) => return Some(value.to_string()),
        None => return None,
    }
}
fn main() {
    let config_file: File;
    match File::open("config.json") {
        Ok(value) => config_file = value,
        Err(_) => {
            println!("缺少配置文件喵");
            std::process::exit(78);
        }
    }
    let config: TraConfig = serde_json::from_reader(config_file).unwrap();
    let input_str = read_to_string("input.srt").unwrap();
    let input_vec: Vec<&str> = input_str.split("\r\n\r\n").collect();   
    let mut file = File::create("output.srt").unwrap();
    for item in input_vec {
        let item_vec: Vec<&str> = item.split(&config.line_type).collect();
        if item_vec.len() > 2 {
            file.write(format!("{}{}{}{}",item_vec[0],&config.line_type,item_vec[1],&config.line_type).as_bytes()).unwrap();
            let mut index = 2;
            while index < item_vec.len() {
                file.write(format!("{}{}",gettra(item_vec[index], &config).unwrap_or("".to_string()),&config.line_type).as_bytes()).unwrap();
                index += 1;
            }
            file.write(format!("{}",&config.line_type).as_bytes()).unwrap();
        }
    }
}
