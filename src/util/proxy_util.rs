#![allow(dead_code)]

use tokio::net::UdpSocket;

use crate::{data_types::ProxyArgs, util::networking_util::check_valid_ip};

pub async fn listen_proxy(listen_ip: &String, listen_port: &String) -> Result<UdpSocket, String> {

    let addr = format!("{}:{}", listen_ip, listen_port);
    let sock = match UdpSocket::bind(addr).await {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("[PROXY] Error creating client socket: {}", e));
        }
    };

    return Ok(sock);
}

pub async fn connect_proxy(target_ip: &String, target_port: &String) -> Result<UdpSocket, String> {

    let addr = format!("{}:{}", target_ip, target_port);
    let socket = match  UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => {s},
        Err(e) => {
            return Err(format!("[PROXY] Error creating server socket: {}", e));
        }
    };
    match socket.connect(addr).await {
        Ok(()) => {},
        Err(e) => {
            return Err(format!("[PROXY] Could not connect to server: {}", e).into());
        }
    };

    return Ok(socket)
}

pub fn validate_proxy_args(args: &ProxyArgs) -> Result<(), String> {
    match check_valid_ip(&args.listen_ip) {
        Ok(()) => {},
        Err(e) => {
            println!();
            return Err(format!("[PROXY] Ip address error: {}", e).into());
        }
    }

    match check_valid_ip(&args.target_ip) {
        Ok(()) => {},
        Err(e) => {
            println!();
            return Err(format!("[PROXY] Ip address error: {}", e).into());
        }
    }

    Ok(())
}
