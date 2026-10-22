use core::str;
use std::fs::{self,OpenOptions};
use std::io::Write;
use std::net::Ipv4Addr;

struct InterfaceConfig {
    name: String,
    address: Ipv4Addr,
    prefix: u8,
}

pub fn ip(args:&[&str]) -> std::io::Result<()> {
    match args {
        ["addr", "show"] => {
            show_ip();
        }
        ["addr", "add", address] => {
            set_ip(address);
        }
        _ => {
            println!("不明なコマンドです");
        }
    }
    Ok(())
}

pub fn show_ip() {
    let path = "/scheme/netcfg/ifaces/eth0/addr/list";

    match fs::read_to_string(path) {
        Ok(ip) => {
            println!("IP address: {}", ip.trim());
        }
        Err(e) => {
            eprintln!("Failed to get IP address: {}", e);
        }
    }
}


pub fn set_ip(address: &str) -> std::io::Result<()> {
    match cheak_ip(address) {
        Ok((ip, subnet)) => {
            let ip_info = InterfaceConfig {
                name: "eth0".to_string(),
                address: ip,
                prefix: subnet,

            };
            let path = "/scheme/nos-ip/addr/set";
            let mut file = OpenOptions::new()
                .write(true)
                .open(path)?;

            writeln!(file, "{}", address)?;
        }
        Err(e) => {
                    eprintln!("Error!!!: {}", e);
                }
    }
    Ok(())
}

fn cheak_ip(address: &str) -> Result<(Ipv4Addr, u8), &'static str> {
    let (ip_addr, subnet) = match address.split_once('/') {
        Some((ip, subnet)) => (ip, subnet),
        None => {
            return Err("IPアドレスの形式が不正です");
        }
    };

    let ip_addr : Ipv4Addr = match ip_addr.parse() {
        Ok(ip) => ip,
        Err(_) => {
            println!("無効なIPアドレスです");
            return Err("IPアドレスの形式が不正です");
        }
    };

    let subnet: u8 = match subnet.parse() {
        Ok(n) if n <= 32 => n,
        _ => {
            println!("無効なプレフィックス長です");
            return Err("IPアドレスの形式が不正です");
        }
    };
    Ok((ip_addr, subnet))
}