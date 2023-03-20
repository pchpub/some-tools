use english_listen_books::mods::types::Config;

fn main() {
    let mut config = Config::new("./");
    config.download().unwrap();
}
