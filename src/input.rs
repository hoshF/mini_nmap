use clap::Parser;

use crate::scanner::Scanner;
use std::{net::IpAddr, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    /// Single ip address
    #[arg(short, long)]
    pub ip: String,

    /// range of ports. Example: 1-1024
    #[arg(short, long)]
    pub range: String,

    /// The timeout in milliseconds before a port is assumed to be closed.
    #[arg(short, long, default_value = "1500")]
    pub timeout: u16,
}

impl Opts {
    pub fn trans(&self) -> Result<Scanner, String> {
        let ip = self.parse_ip()?;
        let (start_port, end_port) = self.parse_range()?;
        let timeout = self.parse_timeout();
        Ok(Scanner::new(ip, start_port, end_port, timeout))
    }

    pub fn parse_ip(&self) -> Result<IpAddr, String> {
        self.ip
            .parse()
            .map_err(|_| format!("{} not valid ip address", self.ip))
    }

    fn parse_range(&self) -> Result<(u16, u16), String> {
        let parse: Vec<&str> = self.range.split('-').collect();

        if parse.len() != 2 {
            return Err("should be two elements like '1-1024'".to_string());
        }

        let start: u16 = parse[0]
            .parse()
            .map_err(|_| format!("{} is invalid", parse[0]))?;
        let end: u16 = parse[1]
            .parse()
            .map_err(|_| format!("{} is invalid", parse[1]))?;

        if start > end {
            return Err("start must less than end".to_string());
        }

        Ok((start, end))
    }

    fn parse_timeout(&self) -> Duration {
        Duration::from_millis(self.timeout.into())
    }
}
