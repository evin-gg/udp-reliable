#![allow(dead_code)]
// standard
use std::net::{IpAddr, Ipv4Addr};

// other util
use get_if_addrs::get_if_addrs;

// ---Universal network utility---

// checks for a valid ip
pub fn check_valid_ip(argpath: &String) -> Result<(), String> {

    let addr: Result<IpAddr, String> = match argpath.parse::<IpAddr>() {
        Ok(ip) => Ok(ip),
        Err(_) => {
            return Err("Invalid IP address".into());
        }
    };

    if addr.clone()?.is_multicast() || addr?.is_unspecified() {
        return Err("IP address not allowed for use".into());
    }

    Ok(())
}

// getifaddrs for rust if needed
pub fn find_address() -> Option<Ipv4Addr> {
    for interface in get_if_addrs().expect("[SERVER] Could not get network interfaces") {
        println!("[SERVER] Interface: {} - IP: {}", interface.name, interface.ip());
        if let IpAddr::V4(ipv4) = interface.ip() {
            if !ipv4.is_loopback() {
                return Some(ipv4)
            }
        }
    }

    return None
}
// --- END ---
