mod input;
mod output;
mod scanner;

use clap::Parser;
use input::Opts;
use output::Output;

fn main() {
    let input = Opts::parse();
    let scanner = match input.trans() {
        Ok(scanner) => scanner,
        Err(e) => panic!("{}", e),
    };
    let list = scanner.scan();
    let ip = match input.parse_ip() {
        Ok(ip) => ip,
        Err(e) => panic!("{}", e),
    };
    let output = Output::new(ip, list);

    println!("{}", output);
}
