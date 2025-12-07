use std::{net::IpAddr, time::Duration};

mod output;
mod scanner;

use output::Output;
use scanner::Scanner;

fn main() {
    let ip: IpAddr = "127.0.0.1".parse().unwrap();

    let scanner = Scanner::new(ip, 0, 10000, Duration::from_millis(500));

    let output = Output::new(ip, scanner.scan());

    println!("{}", output);
}
