#![allow(dead_code)]

use tokio::net::UdpSocket;

pub async fn socket_for_client(listen_ip: &String, listen_port: &String) -> Result<UdpSocket, String> {

    let addr = format!("{}:{}", listen_ip, listen_port);
    let sock = match UdpSocket::bind(addr).await {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("[PROXY] Error creating client socket: {}", e));
        }
    };

    return Ok(sock);
}

pub async fn connect_to_server(target_ip: &String, target_port: &String) -> Result<UdpSocket, String> {

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