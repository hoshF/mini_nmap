use std::collections::HashMap;
use std::fmt;
use std::net::IpAddr;

#[derive(Debug)]
pub struct Output {
    pub socket_list: HashMap<IpAddr, Vec<u16>>,
}

impl Output {
    pub fn new(socket_list: HashMap<IpAddr, Vec<u16>>) -> Self {
        Self { socket_list }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.socket_list.is_empty() {
            write!(f, "No open ports found on any IP")
        } else {
            for (ip, ports) in &self.socket_list {
                if ports.is_empty() {
                    writeln!(f, "IP: {} - No open ports", ip)?;
                } else {
                    let mut sorted_ports = ports.clone();
                    sorted_ports.sort();
                    writeln!(f, "IP: {} - Open ports: {:?}", ip, sorted_ports)?;
                }
            }
            Ok(())
        }
    }
}
