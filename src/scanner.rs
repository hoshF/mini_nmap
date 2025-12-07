use std::{
    io::ErrorKind,
    net::{IpAddr, SocketAddr, TcpStream},
    time::Duration,
};

#[derive(Debug)]
pub struct Scanner {
    ip: IpAddr,
    start_port: u16,
    end_port: u16,
    timeout: Duration,
}

impl Scanner {
    pub fn new(ip: IpAddr, start_port: u16, end_port: u16, timeout: Duration) -> Self {
        Self {
            ip,
            start_port,
            end_port,
            timeout,
        }
    }

    pub fn scan(&self) -> Vec<u16> {
        let mut open_ports = Vec::new();
        let ports: Vec<u16> = (self.start_port..=self.end_port).collect();
        for port in ports {
            let socket = SocketAddr::new(self.ip, port);
            if self.is_port_open(socket) {
                open_ports.push(port);
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
                    println!("Error: {}", e);
                }
                false
            }
        }
    }
}
