use std::{thread, time::Duration};

use clap::{arg, command, Parser};

pub mod sign;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // aliyun short token
    #[arg(short, long)]
    token: String,
}

fn main() {
    let args = Args::parse();
    let token = args.token;
    loop {
        let result = sign::aliyun_auto_sign(&token);
        if result.is_err() {
            println!("auto sign has exception, token:{}", token);
        }
        thread::sleep(Duration::from_secs(12 * 3600));
    }
}
