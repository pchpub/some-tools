use curl::easy::{Easy2, Handler, WriteError, List};

struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        // self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn main() {
    for _i in 0..1 {
        std::thread::spawn(|| {
            let mut easy = Easy2::new(Collector(Vec::new()));
            let mut headers = List::new();
            headers.append("Referer: https://blog.chitang.dev/").unwrap();
            easy.get(true).unwrap();
            easy.ssl_verify_host(false).unwrap();
            easy.ssl_verify_peer(false).unwrap();
            easy.url("https://chitangcos.zyglq.cn/images/howdy/face-add.png")
                .unwrap();
            easy.http_headers(headers).unwrap();
            loop {
                easy.perform().unwrap();
            }
        });
    }
    loop{}
}
