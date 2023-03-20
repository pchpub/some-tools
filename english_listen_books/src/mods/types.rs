use super::request::sync_download;

pub struct Config {
    path: String,
    url_with_name: Vec<(String,String)>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        Config {
            path: path.to_string(),
            url_with_name: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), ()>{
        if self.url_with_name.is_empty() {
            let rdr = std::io::BufReader::new(std::fs::File::open("books.json").unwrap());
            let json_data: serde_json::Value = if let Ok(data) = serde_json::from_reader(rdr) {
                data
            } else {
                return Err(());
            };
            for mp3_data in json_data["mp3"].as_array().unwrap() {
                let url = mp3_data["mp3url"].as_str().unwrap().to_string();
                let name = mp3_data["mp3name"].as_str().unwrap().to_string();
                self.url_with_name.push((url, name));
            }
        }
        Ok(())
    }

    pub fn download(&mut self) -> Result<(), ()> {
        self.init().unwrap();
        for (url, name) in &self.url_with_name {
            let path = format!("{}/{}", self.path, name);
            if let Ok(_) = sync_download(url, &path) {
                println!("Downloaded {}", name);
            } else {
                println!("Failed to download {}", name);
            };
        }

        Ok(())
    }

}

