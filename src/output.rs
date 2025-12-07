use std::fmt;
use std::net::IpAddr;

#[derive(Debug)]
pub struct Output {
    pub ip: IpAddr,
    pub open_ports: Vec<u16>,
}

impl Output {
    pub fn new(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        Self { ip, open_ports }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.open_ports.is_empty() {
            write!(f, "IP: {} - No open ports found", self.ip)
        } else {
            write!(f, "IP: {} - Open ports: {:?}", self.ip, self.open_ports)
        }
    }
}
