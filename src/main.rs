use std::env;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process;

fn ip_reverse(ip: &IpAddr) -> String {
    match ip {
        IpAddr::V4(ip4) => ip4_reverse(ip4, "in-addr.arpa"),
        IpAddr::V6(ip6) => ip6_reverse(ip6, "ip6.arpa"),
    }
}

fn ip4_reverse(ip: &Ipv4Addr, suffix: &str) -> String {
    let mut rev = String::with_capacity(16 + suffix.len());
    for oct in ip.octets().iter().rev() {
        rev.push_str(format!("{}.", oct).as_str());
    }
    rev.push_str(suffix);
    rev
}

fn ip6_reverse(ip: &Ipv6Addr, suffix: &str) -> String {
    let mut rev = String::with_capacity(64 + suffix.len());
    for oct in ip.octets().iter().rev() {
        rev.push_str(format!("{:x}.", oct & 0x0fu8).as_str());
        // Rust gets mad about right-shifting set bits, so we need to mask + shift
        rev.push_str(format!("{:x}.", (oct & 0xf0u8) >> 4u8).as_str());
    }
    rev.push_str(suffix);
    rev
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args[1..].iter() {
        let ip: Result<IpAddr, _> = arg.parse();
        let rev = match ip {
            Ok(ip) => ip_reverse(&ip),
            Err(e) => {
                eprintln!("Unable to parse arguments: {}", e);
                process::exit(1);
            }
        };
        println!("{}", rev);
    }
}
