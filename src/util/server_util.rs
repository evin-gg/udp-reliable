#![allow(dead_code)]
// use std::io::Error;

// standard sockets and addresses
use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};

// network sockets
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

// other util
use bincode::{config};

// data types
use crate::data_types::{Message, ServerArgs};

// ---Server Setup functions---

// correct amount of server args
pub fn server_arg_validation(args: &Vec<String>) -> Result<(), String> {
    if args.len() != 3 {
        return Err("Usage: <listen-ip> <listen-port>".into());
    }

    else {
        Ok(()) 
    }
}

pub fn setup_server(args: &ServerArgs) -> Result<Socket, String> {
    let local_ip: IpAddr = args.target_ip.parse().unwrap();

    let port: u16 = args.target_port;
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

pub fn send_ack(server_socket: &UdpSocket, client_addr: &SocketAddr, seq_number: u8) -> Result<(), String> {
    let buf = [seq_number];

    match server_socket.send_to(&buf, client_addr) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("[SERVER] Failed to send ACK: {}", e)),
    }
}
// --- END ---