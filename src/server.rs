mod cipher;
mod networking_util;

use std::io::Bytes;
// standard
// use ::std::os::fd::AsRawFd;
// use std::result;
use ::std::{env, process};
// use bincode::config;
use ctrlc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

// network sockets
use socket2::Socket;

// poll
use tokio::net::{ UdpSocket };
// use tokio::io::{AsyncReadExt};

// other util
use networking_util::{send_ack, check_valid_ip, server_arg_validation, setup_server};

use crate::networking_util::Message;
use crate::networking_util::deserialize_message;

fn handle_signal(flag: &Arc<AtomicBool>) {
    println!("Signal received");
    flag.store(false, Ordering::SeqCst);
}

#[tokio::main]
async fn main() {
    let catch = Arc::new(AtomicBool::new(true));
    let c = catch.clone();

    ctrlc::set_handler(move || handle_signal(&c)).expect("[SERVER] Signal Handler Error");

    // args
    let args: Vec<String> = env::args().collect();

    // verify args
    match server_arg_validation(&args) {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

    //verify ip
    match check_valid_ip(&args[1]) {
        Ok(()) => {}
        Err(e) => {
            println!("Ip address error: {}", e);
            process::exit(1);
        }
    }

    // setup server
    let socket: Socket = match setup_server(&args) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
    
    match socket.set_nonblocking(true) {
        Ok(()) => {},
        Err(e) => {
            println!("[SERVER] Failed to set socket to non-blocking: {}", e);
            process::exit(1);
        }
    };

    // convert socket2 to UDP stream
    let listener: std::net::UdpSocket = socket.into();
    let tokio_listener = match UdpSocket::from_std(listener) {
        Ok(listener) => {listener},
        Err(e) => {
            eprintln!("[SERVER] Failed to convert socket to a Tokio listener: {}", e);
            process::exit(1);
        }
    };


    while catch.load(Ordering::SeqCst) {
        let mut buf = [0u8; 1024];

        // new accept
        let packet= tokio_listener.recv_from(&mut buf);

        let result = packet.await;
        match result {
            Ok((n, addr)) => {
                println!("[SERVER] Received {} bytes from {}", n, addr);
                
                let (data, _data_size) = match deserialize_message(&buf[0..n]) {
                    Ok(d) => d,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                println!("{}, SEQ: {}", data.message.trim_end(), data.seq_number);

                // send back ack
                let ack_buf = [data.seq_number];
                let bytes = tokio_listener.send_to(&ack_buf, addr).await;

                println!("Sent {} bytes", bytes.unwrap())
            }
            Err(e) => {
                eprintln!("[SERVER] recv_from failed: {}", e);
            }
        }
    }

    println!("[SERVER] Socket closed. Exiting");
}
