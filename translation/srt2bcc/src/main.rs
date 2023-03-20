use std::{fs::{File, read_to_string}, env, io::Write};
use serde::{Deserialize, Serialize};
use std::path::Path;
use regex::Regex;

#[derive(Serialize, Deserialize)]
struct TruConfig {
    line_type : String,
    extra_content : String,
    time : f64,
}

#[derive(Serialize, Deserialize)]
struct Bccitem {
    content: String,
    location: i32,
    from: f64,
    to: f64,
}

#[derive(Serialize, Deserialize)]
struct Bcc {
    body : Vec<Bccitem>,
}
fn main() {
    let config_file: File;
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    match File::open("config.json") {
        Ok(value) => config_file = value,
        Err(_) => {
            println!("缺少配置文件喵");
            std::process::exit(78);
        }
    }
    let config: TruConfig = serde_json::from_reader(config_file).unwrap();
    let re = Regex::new(r"(\d{2,10}):(\d{2}):(\d{2})[,\.](\d{3}) --> (\d{2,10}):(\d{2}):(\d{2})[,\.](\d{3})").unwrap();
    for srtfiles in args {
        let input_path = Path::new(&srtfiles);
        println!("{}", srtfiles);
        let input_str = read_to_string(&srtfiles).unwrap();
        let input_vec: Vec<&str> = input_str.split("\r\n\r\n").collect();
        let file_stem = input_path.file_stem().unwrap();
        let mut file = File::create(format!("{}.bcc",file_stem.to_str().unwrap())).unwrap();
        let mut bcc_items: Vec<Bccitem> = Vec::new();
        let mut replace = true;
        let mut end_time_last = 0.0;
        for item in input_vec {
            let item_vec: Vec<&str> = item.split(&config.line_type).collect();
            if item_vec.len() > 2 {
                let mut index = 2;
                let mut body = String::new();
                while index < item_vec.len() {
                    body.push_str(item_vec[index]);
                    index += 1;
                    if index != item_vec.len() {
                        body.push_str("\n");
                    }
                }
                let caps = re.captures(item_vec[1]).unwrap();
                let start_time = caps.get(1).unwrap().as_str().to_string().parse::<f64>().unwrap()*60.0*60.0 + caps.get(2).unwrap().as_str().to_string().parse::<f64>().unwrap()*60.0 +caps.get(3).unwrap().as_str().to_string().parse::<f64>().unwrap() +caps.get(4).unwrap().as_str().to_string().parse::<f64>().unwrap()/1000.0 ;
                let end_time = caps.get(5).unwrap().as_str().to_string().parse::<f64>().unwrap()*60.0*60.0 + caps.get(6).unwrap().as_str().to_string().parse::<f64>().unwrap()*60.0 +caps.get(7).unwrap().as_str().to_string().parse::<f64>().unwrap() +caps.get(8).unwrap().as_str().to_string().parse::<f64>().unwrap()/1000.0 ;
                if replace {
                    if start_time >= config.time {
                        bcc_items.push(Bccitem { content: config.extra_content.clone(), location: 2, from: end_time_last, to: config.time });
                    }else if end_time <= config.time {
                        if end_time_last != start_time {
                            bcc_items.push(Bccitem { content: config.extra_content.clone(), location: 2, from: end_time_last, to: start_time });
                        }
                        bcc_items.push(Bccitem { content: format!("{body}\n{}",config.extra_content), location: 2, from: start_time, to: end_time });
                    }else if start_time < config.time && end_time >= config.time {
                        if end_time_last != start_time {
                            bcc_items.push(Bccitem { content: config.extra_content.clone(), location: 2, from: end_time_last, to: start_time });
                        }
                        bcc_items.push(Bccitem { content: format!("{body}\n{}",config.extra_content), location: 2, from: start_time, to: config.time });
                        bcc_items.push(Bccitem { content: body, location: 2, from: config.time, to: end_time });
                    }
                    end_time_last = end_time;
                    if end_time >= config.time {
                        replace = false;
                    }
                }else{
                    bcc_items.push(Bccitem { content: body, location: 2, from: start_time, to: end_time });
                }
                
            }
        }
        let bcc = Bcc {body: bcc_items};
        file.write(serde_json::to_string(&bcc).unwrap().as_bytes()).unwrap();
    }
}
