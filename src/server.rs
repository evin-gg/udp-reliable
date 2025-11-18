mod cipher;
mod networking_util;

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
use networking_util::{check_valid_ip, server_arg_validation, setup_server};

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

                println!("{}", data.message)
            }
            Err(e) => {
                eprintln!("[SERVER] recv_from failed: {}", e);
            }
        }

        // tokio::spawn(async move {
        //     let mut buf = [0u8; 1024];

        //     loop {
        //         let _n = match clientfd.read(&mut buf).await {
        //             Ok(0) => return,
        //             Ok(_n) => {
        //                 println!("[SERVER] Client ID: {} connected", clientfd.as_raw_fd()); 
        //                 println!("[SERVER] Payload: {}", String::from_utf8_lossy(&buf[.._n]));
        //             },
        //             Err(e) => {
        //                 eprintln!("[SERVER] Could not read from client: {}", e);
        //                 return;
        //             }
        //         };

        //         let (response, _len): (Message, usize) = match bincode::decode_from_slice(&buf, config::standard()) {
        //             Ok(r) => r,
        //             Err(e) => {
        //                 println!("{}", e);
        //                 process::exit(1);
        //             }
        //         };

        //         println!("{}", response.message);
                
        //         // let n: u64 = rand::random_range(0..4);
        //         // std::thread::sleep(Duration::from_secs(n));

        //         // send(clientfd.as_raw_fd(), response.as_bytes(), MsgFlags::empty()).expect("[SERVER] Error sending response");
        //     }
        // });
    }

    println!("[SERVER] Socket closed. Exiting");
}
