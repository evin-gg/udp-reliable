#![allow(dead_code)]

use std::time::Duration;

use crate::{data_types::ProxyArgs, util::networking_util::check_valid_ip};
use rand::Rng;
use tokio::net::UdpSocket;

pub async fn listen_proxy(listen_ip: &String, listen_port: &String) -> Result<UdpSocket, String> {
    let addr = format!("{}:{}", listen_ip, listen_port);
    let sock = match UdpSocket::bind(addr).await {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("[PROXY] Error binding proxy socket for client: {}", e));
        }
    };

    return Ok(sock);
}

pub async fn connect_proxy(target_ip: &String, target_port: &String) -> Result<UdpSocket, String> {
    let addr = format!("{}:{}", target_ip, target_port);
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("[PROXY] Error binding proxy socket for server: {}", e));
        }
    };
    match socket.connect(addr).await {
        Ok(()) => {}
        Err(e) => {
            return Err(format!("[PROXY] Could not connect to server: {}", e).into());
        }
    };

    return Ok(socket);
}

pub fn validate_proxy_args(args: &ProxyArgs) -> Result<(), String> {
    match check_valid_ip(&args.listen_ip) {
        Ok(()) => {}
        Err(e) => {
            println!();
            return Err(format!("[PROXY] Ip address error: {}", e).into());
        }
    }

    match check_valid_ip(&args.target_ip) {
        Ok(()) => {}
        Err(e) => {
            println!();
            return Err(format!("[PROXY] Ip address error: {}", e).into());
        }
    }

    Ok(())
}

pub fn chance(chance: u32) -> bool {
    let mut rng = rand::rng();
    let mut roll: u32 = rng.random_range(..100);
    roll += 1;
    roll > chance
}

pub fn random_delay(min_delay: u32, max_delay: u32) -> () {
    let mut rng = rand::rng();
    let delay_length = rng.random_range(min_delay..=max_delay);
    println!("[PROXY] Client packet delayed by {} ms", delay_length);
    std::thread::sleep(Duration::from_millis(delay_length.into()));
}

pub async fn forward_packet_server(
    server_socket: &UdpSocket,
    client_incoming: &[u8; 1024],
    n: usize,
) -> () {
    let sent = server_socket.send(&client_incoming[0..n]).await;
    println!("[PROXY] {} bytes PROXY -> SERVER", sent.unwrap());
}

pub async fn forward_packet_client(
    listening_socket: &UdpSocket,
    client_addr: &Option<std::net::SocketAddr>,
    server_incoming: &[u8],
    n: usize,
) {
    let sent = listening_socket
        .send_to(&server_incoming[..n], client_addr.unwrap())
        .await
        .unwrap();

    println!("[PROXY] {} bytes CLIENT <- PROXY", sent);
}

