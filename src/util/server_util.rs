#![allow(dead_code)]
// use std::io::Error;

// standard sockets and addresses
use std::net::{IpAddr, SocketAddrV4, SocketAddrV6};

// network sockets
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

// other util
use bincode::{config};

// data types
use crate::data_types::{Message, ServerArgs};

// ---Server Setup functions---
pub fn setup_server(args: &ServerArgs) -> Result<Socket, String> {
    let local_ip: IpAddr = args.listen_ip.parse().unwrap();

    let port: u16 = args.listen_port;
    let (domain, addr) = match local_ip {
        IpAddr::V4(v4) => (Domain::IPV4, SockAddr::from(SocketAddrV4::new(v4, port))),
        IpAddr::V6(v6) => (Domain::IPV6, SockAddr::from(SocketAddrV6::new(v6, port, 0, 0))),
    };


    let socket = match Socket::new(domain, Type::DGRAM, Some(Protocol::UDP)) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("[SERVER] Socket creation failed: {}", e))
        }
    };

    match socket.bind(&addr) {
        Ok(()) => {},
        Err(e) => {
            return Err(format!("[SERVER] Bind failed: {}", e))
        }
    }

    let local_addr = socket.local_addr().expect("[SERVER] Could not get local address");
    let std_addr = local_addr.as_socket().unwrap();
    println!("[SERVER] Server listening on {}", std_addr);

    return Ok(socket)
}

pub fn deserialize_message(buffer: &[u8]) -> Result<(Message, usize), String> {

    let data = match bincode::decode_from_slice::<Message, _>(buffer, config::standard()) {
        Ok(d) => d,
        Err(e) => {
            return Err(format!("[SERVER] Decode error: {}", e))
        }
    };
    return Ok(data);
}

pub async fn send_ack(sock: &tokio::net::UdpSocket, seq: u8, addr: std::net::SocketAddr) {
    let ack_buf = [seq];

    let _bytes = sock.send_to(&ack_buf, addr).await;
    println!("[SERVER] Sent ACK");
}

// --- END ---
