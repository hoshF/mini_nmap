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
    let output = Output::new(list);

    println!("{}", output);
}
