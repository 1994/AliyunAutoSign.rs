use std::{thread, time::Duration};

use clap::{arg, command, Parser};
use config::Config;
use message::send_message;

pub mod config;
pub mod message;
pub mod sign;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // config file location
    #[arg(short, long)]
    config: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config_file = args.config;
    let config = Config::new(&config_file)?;
    let aliyun_token = config.aliyun.token;
    loop {
        let result = sign::aliyun_auto_sign(&aliyun_token);
        if result.is_err() {
            println!("auto sign has exception, token:{}", aliyun_token);
        } else {
            let send_response = send_message(
                &config.telegram.chat_id,
                &config.telegram.token,
                "aliyun auto sign success",
            );
            if send_response.is_err() {
                eprintln!(
                    "auto sign send message failed, {}",
                    send_response.err().unwrap()
                );
            }
        }

        thread::sleep(Duration::from_secs(12 * 3600));
    }
}
