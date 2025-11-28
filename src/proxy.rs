mod data_types;
mod util;

// signal handling
use ctrlc;
use tokio::net::UdpSocket;
use std::fs::File;
use std::process;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

// data types
use data_types::{ProxyArgs};

// functions and utility
use crate::util::proxy_util::*;
use crate::util::networking_util::{check_valid_ip};
use rand::Rng;
use std::time::Duration;
use std::io::Write;

fn handle_signal(flag: &Arc<AtomicBool>) {
    println!("Signal received");
    flag.store(false, Ordering::SeqCst);
}
// sent ack retries lost

#[tokio::main]
async fn main() {
    let catch = Arc::new(AtomicBool::new(true));
    let c = catch.clone();
    ctrlc::set_handler(move || handle_signal(&c)).expect("[SERVER] Signal Handler Error");

    let args: ProxyArgs = argh::from_env();

    let mut client_incoming = [0u8; 1024];
    let mut server_incoming = [0u8; 1024];

    let mut file = match File::create("log.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("[PROXY] Could not create log file: {}", e);
            process::exit(1);
        }
    };

    match check_valid_ip(&args.target_ip) {
        Ok(()) => {},
        Err(e) => {
            println!("[PROXY] Ip address error: {}", e);
            process::exit(1);
        }
    }

    let listening_socket: UdpSocket = match socket_for_client(&args.listen_ip, &args.listen_port).await {
        Ok(fd) => fd,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let server_socket: UdpSocket = match connect_to_server(&args.target_ip, &args.target_port).await {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    // std::process::Command::new("clear").status().unwrap();
    println!("[PROXY] Proxy server running");
    let mut client_addr: Option<std::net::SocketAddr> = None;

    loop {
        tokio::select! {
            event1 = listening_socket.recv_from(&mut client_incoming) => {

                _ = writeln!(file, "[SENT]");

                let (n, addr) = event1.unwrap();
                println!("\n[PROXY] I received {} bytes from client at {}", n, addr);

                // apply delay chance or drop chance
                let mut rng = rand::rng();
                let mut drop_roll: u32 = rng.random_range(..100);
                drop_roll += 1;
                println!("CLIENT DROP Rolled {} >= {}", drop_roll, args.client_drop);

                if drop_roll >= args.client_drop {
                    println!("[PROXY] Client packet stays");
                    let mut rng = rand::rng();
                    let mut delay_roll: u32 = rng.random_range(..100);
                    delay_roll += 1;
                    println!("CLIENT DELAY Rolled {} >= {}", delay_roll, args.client_delay);

                    if delay_roll <= args.client_delay {
                        
                        let mut rng = rand::rng();
                        let delay_length = rng.random_range(args.client_delay_time_min..=args.client_delay_time_max);
                        println!("[PROXY] Client packet delayed by {} ms", delay_length);
                        std::thread::sleep(Duration::from_millis(delay_length.into()));
                        
                    } else {
                        println!("[PROXY] Client packet not delayed");
                    }
                    
                    
                    let sent = server_socket.send(&client_incoming[0..n]).await;
                    println!("[PROXY] I forwarded {} bytes to the server", sent.unwrap());
                } else {
                    println!("[PROXY] Client packet dropped");
                }

                client_addr = Some(addr);
            }

            event2 = server_socket.recv(&mut server_incoming) => {
                let n = event2.unwrap();
                println!("[PROXY] I received {} bytes from the server", n);

                

                // apply delay chance or drop chance
                let mut rng = rand::rng();
                let mut drop_roll: u32 = rng.random_range(..100);
                drop_roll += 1;
                println!("SERVER DROP Rolled {} >= {}", drop_roll, args.server_drop);
                if drop_roll >= args.server_drop {
                    println!("[PROXY] Server packet stays");
                    let mut rng = rand::rng();
                    let mut delay_roll: u32 = rng.random_range(..100);
                    delay_roll += 1;
                    println!("SERVER DELAY Rolled {} >= {}", delay_roll, args.server_delay);

                    if delay_roll <= args.server_delay {
                        let mut rng = rand::rng();
                        let delay_length = rng.random_range(args.server_delay_time_min..=args.server_delay_time_max);
                        println!("[PROXY] Server packet delayed by {} ms", delay_length);
                        std::thread::sleep(Duration::from_millis(delay_length.into()));
                    } else {
                        println!("[PROXY] Server packet not delayed");
                    }
                    
                    
                    let sent = listening_socket.send_to(&server_incoming[0..n], client_addr.unwrap()).await.unwrap(); 
                    println!("[PROXY] I forwarded {} bytes to the client at {}", sent, client_addr.unwrap());
                    _ = writeln!(file, "[ACK]");
                } else {
                    println!("[PROXY] Client packet dropped");
                }

                
            }
        }
    }

    }
