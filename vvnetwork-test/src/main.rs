use fast_socks5::client::Config;
use fast_socks5::{client::Socks5Stream, Result};
use log::{warn};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    use chrono::Local;
    use std::io::Write;
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}][{:>5}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_styled_level(record.level()),
                &record.args()
            )
        })
        .init();

    warn!("vvnetwork test started");
    loop {
        spawn_socks_client("127.0.0.1:7777", "20.205.8.151", 80)
            .await
            .unwrap_or_default();
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
}

async fn spawn_socks_client(socks_server: &str, target_addr: &str, target_port: u16) -> Result<()> {
    #[derive(Debug)]
    struct TestConfig {
        socks_server: String,
        target_addr: String,
        target_port: u16,
        username: Option<String>,
        password: Option<String>,
        skip_auth: bool,
    }

    let test_config = TestConfig {
        socks_server: socks_server.to_owned(),
        target_addr: target_addr.to_owned(),
        target_port,
        username: Some("Clash".to_owned()),
        password: Some("AHgnCTRv".to_owned()),
        skip_auth: false,
    };
    let mut socks;
    let mut config = Config::default();
    config.set_skip_auth(test_config.skip_auth);

    // Creating a SOCKS stream to the target address thru the socks server
    if test_config.username.is_some() {
        socks = Socks5Stream::connect_with_password(
            test_config.socks_server,
            test_config.target_addr,
            test_config.target_port,
            test_config.username.unwrap(),
            test_config.password.expect("Please fill the password"),
            config,
        )
        .await?;
    } else {
        socks = Socks5Stream::connect(
            test_config.socks_server,
            test_config.target_addr,
            test_config.target_port,
            config,
        )
        .await
        .unwrap();
    }
    long_request(&mut socks).await?;

    Ok(())
}

/// Simple HTTP request
async fn long_request<T: AsyncRead + AsyncWrite + Unpin>(stream: &mut T) -> Result<()> {
    let mut send_data = 1;
    let mut is_first = true;
    loop {
        if send_data == 100 {
            send_data = 1;
        }
        let send_data_bytes = vec![send_data as u8];
        if let Err(value) = stream.write_all(&send_data_bytes).await {
            warn!("vvnetwork down");
            return Err(value.into());
        };
        let mut result = [0u8; 1024];
        if let Err(value) = stream.read(&mut result).await {
            warn!("vvnetwork down");
            return Err(value.into());
        };
        let return_num = result[0];
        if return_num != send_data {
            break;
        } else if is_first {
            is_first = false;
            warn!("vvnetwork up");
        }
        send_data += 1;
        // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    warn!("vvnetwork down");
    Ok(())
}
