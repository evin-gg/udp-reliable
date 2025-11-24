#![allow(dead_code)]
// use std::io::Error;
// standard
use std::os::fd::AsRawFd;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6, UdpSocket};
use std::time::Duration;

use nix::libc::PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP;
// network sockets
use nix::sys::socket::{
    MsgFlags, recv
};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
// use std::fs::File;
// use tokio::io::unix::AsyncFd;
// use std::net::TcpStream;

// other util
use get_if_addrs::get_if_addrs;
use bincode::{Decode, Encode, config};

#[derive(Encode, Decode)]
pub struct Message {
    pub seq_number: u8,
    pub message: String,
}

// ---Client Setup functions---

// Validates arg count (variable)
pub fn client_arg_validation(args: &Vec<String>) -> Result<(), String> {
    if args.len() != 5 {
        return Err("[CLIENT] Usage: <target-ip> <target-port> <timeout> <max-retries>".to_string());
    }

    // for i in args[2].chars() {
    //     if !i.is_ascii_alphabetic() {
    //         return Err("[CLIENT] the key must be ascii alphabetic".to_string());
    //     }
    // }
    
    Ok(())
}

pub fn client_connect(args: &Vec<String>) -> Result<Socket, String> {

    println!("Connecting to addr: {}", args[1]);
    println!("Port: {}", args[2]);
    let local_ip: IpAddr = args[1].parse().unwrap();
    let port: u16 = match args[2].parse() {
        Ok(p) => p,
        Err(_) => return Err("[SERVER] Invalid port".to_string()),
    };

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
                println!("Received Ack/SeqNumber = {}", buf[0]);
                return Ok(())
            }
            Err(_e) => {
                println!("[CLIENT] No ACK, Retrying..({})", n);
                _ = send_message(serverfd, data);
            }
        }   

        std::thread::sleep(Duration::from_secs(timeout));
    }

    return Err(("[CLIENT] Did not receive ACK.").into());
    
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

pub fn setup_server(args: &Vec<String>) -> Result<Socket, String> {
    let local_ip: IpAddr = args[1].parse().unwrap();

    let port: u16 = match args[2].parse() {
        Ok(p) => p,
        Err(_) => return Err("[SERVER] Invalid port".to_string()),
    };

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

    //not on udp
    // match socket.listen(5) {
    //     Ok(()) => {},
    //     Err(e) => {
    //         return Err(format!("[SERVER] Listen failed: {}", e))
    //     }
    // }

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


// ---Universal---

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
