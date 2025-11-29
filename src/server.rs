mod util;
mod data_types;

// signal handling
use ctrlc;
use std::process;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

// network sockets
use socket2::Socket;

// poll
use tokio::net::{ UdpSocket };

// functions and utility
use data_types::{ServerArgs};
use crate::util::server_util::*;
use crate::util::networking_util::check_valid_ip;
use std::io::Write;
use std::fs::File;

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
    let args: ServerArgs = argh::from_env();

    //verify ip
    match check_valid_ip(&args.target_ip) {
        Ok(()) => {}
        Err(e) => {
            println!("Ip address error: {}", e);
            process::exit(1);
        }
    }

    let mut file = match File::create("./loggers/logs/server.log") {
        Ok(f) => f,
        Err(e) => {
            println!("[SERVER] Could not create log file: {}", e);
            process::exit(1);
        }
    };


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

    std::process::Command::new("clear").status().unwrap();
    println!("[SERVER] Running");
    while catch.load(Ordering::SeqCst) {
        let mut buf = [0u8; 1024];

        // new accept
        let packet= tokio_listener.recv_from(&mut buf);

        let result = packet.await;
        match result {
            Ok((n, addr)) => {
                std::process::Command::new("clear").status().unwrap();
                println!("[SERVER] Received {} bytes from {}", n, addr);
                _ = writeln!(file, "[RECEIVED]");
                
                let (data, _data_size) = match deserialize_message(&buf[0..n]) {
                    Ok(d) => d,
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                };

                println!("Message: {}", data.message.trim_end());
                println!("Sequence #: {}\n", data.seq_number);

                // send back ack
                let ack_buf = [data.seq_number];
                let _bytes = tokio_listener.send_to(&ack_buf, addr).await;
                println!("[SERVER] Sent ACK");
                _ = writeln!(file, "[ACK]");
            }
            Err(e) => {
                eprintln!("[SERVER] recv_from failed: {}", e);
            }
        }
    }

    println!("[SERVER] Socket closed. Exiting");
}
