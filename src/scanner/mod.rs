mod socket_iterator;
use socket_iterator::SocketIterator;

use std::{
    collections::HashMap,
    io::ErrorKind,
    net::{IpAddr, SocketAddr, TcpStream},
    time::Duration,
};

#[derive(Debug)]
pub struct Scanner {
    pub ips: Vec<IpAddr>,
    pub start_port: u16,
    pub end_port: u16,
    pub timeout: Duration,
}

impl Scanner {
    pub fn new(ips: Vec<IpAddr>, start_port: u16, end_port: u16, timeout: Duration) -> Self {
        Self {
            ips,
            start_port,
            end_port,
            timeout,
        }
    }

    pub fn scan(&self) -> HashMap<IpAddr, Vec<u16>> {
        let mut open_ports: HashMap<IpAddr, Vec<u16>> = HashMap::new();
        let ports: Vec<u16> = (self.start_port..=self.end_port).collect();
        let socket_iterator = SocketIterator::new(&self.ips, &ports);

        for socket in socket_iterator {
            if self.is_port_open(socket) {
                open_ports
                    .entry(socket.ip())
                    .or_default()
                    .push(socket.port());
            }
        }

        open_ports
    }

    pub fn is_port_open(&self, socket: SocketAddr) -> bool {
        match TcpStream::connect_timeout(&socket, self.timeout) {
            Ok(stream) => {
                drop(stream);
                true
            }
            Err(e) => {
                if e.kind() != ErrorKind::ConnectionRefused {
                    println!("TCP Scan Error: {}", e);
                }
                false
            }
        }
    }
}
