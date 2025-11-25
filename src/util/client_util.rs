#![allow(dead_code)]

// standard network sockets and addresses
use std::os::fd::AsRawFd;
use std::net::{IpAddr, SocketAddrV4, SocketAddrV6, UdpSocket};

// time
use std::time::Duration;

// some nix flags
use nix::sys::socket::{
    MsgFlags, recv
};

// network sockets
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

// data types
use crate::data_types::{ClientArgs, Message};

// ---Client Setup functions---
pub fn client_connect(args: &ClientArgs) -> Result<Socket, String> {

    println!("Connecting to addr: {}", args.target_ip);
    println!("Port: {}", args.target_port);
    let local_ip: IpAddr = args.target_ip.parse().unwrap();
    let port: u16 = args.target_port;

    let (domain, addr) = match local_ip {
        IpAddr::V4(v4) => (Domain::IPV4, SockAddr::from(SocketAddrV4::new(v4, port))),
        IpAddr::V6(v6) => (Domain::IPV6, SockAddr::from(SocketAddrV6::new(v6, port, 0, 0))),
    };
    
    let socket = match Socket::new(domain, Type::DGRAM, Some(Protocol::UDP)) {
        Ok(s) => {s},
        Err(_e) => return Err("[CLIENT] Socket Creation Error".into())
    };
    
    match socket.connect(&addr){
        Ok(()) => {},
        Err(_e) => {return Err("[CLIENT] Error Connecting to Server".into())}
    };

    


    println!("[CLIENT] Connected to server");
    return Ok(socket);
}

// sends the message
pub fn send_message(serverfd: &UdpSocket, data: &Message) -> Result<(), String> {

    let byte_buffer= match bincode::encode_to_vec(&data, bincode::config::standard()) {
        Ok(b) => b,
        Err(_e) => {return Err("[CLIENT] could not serialize message".into())},
    };

    match serverfd.send(&byte_buffer) {
        Ok(_) => {},
        Err(_e) => {
            return Err("[CLIENT] could not send message".into())
        }
    };

    Ok(())
}

pub fn wait_ack(serverfd: &UdpSocket, data: &Message, timeout: u64, retries: i32) -> Result<(), String> {
    let cloned_fd = serverfd.try_clone().unwrap();
    let std_socket: UdpSocket = cloned_fd.into();

    std_socket.set_nonblocking(true).unwrap();

    let mut buf = [0u8];

    std::thread::sleep(Duration::from_secs(1));
    for n in 0..retries {
        match std_socket.recv(&mut buf) {
            Ok(_len) => {
                println!("[CLIENT] Received ACK/Sequence Number = {}", buf[0]);
                return Ok(())
            }
            Err(_e) => {
                println!("[CLIENT] No ACK, Retrying..({})", n);
                _ = send_message(serverfd, data);
            }
        }   

        std::thread::sleep(Duration::from_secs(timeout));
    }

    return Err(("[CLIENT] Did not receive ACK. Continuing Program").into());
    
}

// Reading a response
pub fn client_response_handler(socket: &Socket) { 
    let mut buffer = [0u8; 1024];
    let _read_bytes = match recv(socket.as_raw_fd(), &mut buffer, MsgFlags::empty()) {
        Ok(b) => {b},
        Err(_b) => {
            println!("Bytes not received");
            return;
        }
    };

    println!("Message from server: {}", String::from_utf8_lossy(&buffer));
}

// --- END ---