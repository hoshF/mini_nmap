use crate::input::Opts;
use cidr_utils::cidr::IpInet;
use std::{net::IpAddr, str::FromStr};

pub fn parse_addresses(input: &Opts) -> Vec<IpAddr> {
    input
        .addresses
        .iter()
        .flat_map(|address| parse_address(address))
        .collect()
}

fn parse_address(address: &str) -> Vec<IpAddr> {
    if let Ok(addr) = IpAddr::from_str(address) {
        vec![addr]
    } else if let Ok(net_addr) = IpInet::from_str(address) {
        net_addr.network().into_iter().addresses().collect()
    } else {
        eprintln!("{} cant format to ip", address);
        vec![]
    }
}
