mod data_types;
mod util;

// signal handling
use ctrlc;
use std::process;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::net::UdpSocket;

// data types
use data_types::ProxyArgs;

// functions and utility
use crate::util::proxy_util::{
    drop_chance, delay_chance, connect_proxy, forward_packet_client, forward_packet_server, listen_proxy,
    random_delay, validate_proxy_args,
};
use std::fs::File;
use std::io::Write;

fn handle_signal(flag: &Arc<AtomicBool>) {
    println!("Signal received");
    flag.store(false, Ordering::SeqCst);
}

#[tokio::main]
async fn main() {
    let catch = Arc::new(AtomicBool::new(true));
    let c = catch.clone();
    ctrlc::set_handler(move || handle_signal(&c)).expect("[SERVER] Signal Handler Error");

    let args: ProxyArgs = argh::from_env();

    let mut client_incoming = [0u8; 1024];
    let mut server_incoming = [0u8; 1024];

    let mut file = match File::create("./loggers/logs/proxy.log") {
        Ok(f) => f,
        Err(e) => {
            println!("[PROXY] Could not create log file: {}", e);
            process::exit(1);
        }
    };

    match validate_proxy_args(&args) {
        Ok(()) => {}
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }

    let listening_socket: UdpSocket = match listen_proxy(&args.listen_ip, &args.listen_port).await {
        Ok(fd) => fd,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let server_socket: UdpSocket = match connect_proxy(&args.target_ip, &args.target_port).await {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let server_socket = Arc::new(server_socket);
    let listening_socket = Arc::new(listening_socket);

    std::process::Command::new("clear").status().unwrap();
    println!("Client Drop Chance: %{}", args.client_drop);
    println!("Server Drop Chance: %{}", args.server_drop);
    println!("Client Delay Chance: %{}", args.client_delay);
    println!("Server Delay Chance: %{}", args.server_delay);
    println!(
        "Client Delay Time Range:{}ms - {}ms",
        args.client_delay_time_min, args.client_delay_time_max
    );
    println!(
        "Server Delay Time Range:{}ms - {}ms",
        args.server_delay_time_min, args.server_delay_time_max
    );
    println!("[PROXY] Proxy server running");
    let mut client_addr: Option<std::net::SocketAddr> = None;

    // while catch.load(Ordering::SeqCst) {
    //     tokio::select! {
    //         event1 = listening_socket.recv_from(&mut client_incoming) => {

    //             _ = writeln!(file, "[SENT]");

    //             let (n, addr) = event1.unwrap();
    //             println!("\n----------------MESSAGE----------------");
    //             println!("[PROXY] {} bytes CLIENT -> PROXY", n);

    //             if chance(args.client_drop) {
    //                 println!("[PROXY] Client packet stays");

    //                 if chance(args.client_delay) {
    //                     random_delay(args.client_delay_time_min, args.client_delay_time_max);

    //                 } else {
    //                     println!("[PROXY] Client packet not delayed");
    //                 }
    //                 forward_packet_server(&server_socket, &client_incoming, n).await;
    //             } else {
    //                 println!("[PROXY] Client packet dropped");
    //             }

    //             client_addr = Some(addr);
    //         }

    //         event2 = server_socket.recv(&mut server_incoming) => {
    //             let n = event2.unwrap();
    //             println!("[PROXY] {} bytes PROXY <- SERVER", n);

    //             if chance(args.server_drop) {
    //                 println!("[PROXY] Server packet stays");

    //                 if chance(args.server_delay) {
    //                     random_delay(args.server_delay_time_min, args.server_delay_time_max);
    //                 } else {
    //                     println!("[PROXY] Server packet not delayed");
    //                 }

    //                 forward_packet_client(
    //                     &listening_socket,
    //                     &client_addr,
    //                     &server_incoming,
    //                     n
    //                 ).await;
    //                 _ = writeln!(file, "[ACK]");
    //             } else {
    //                 println!("[PROXY] Client packet dropped");
    //             }

    //         }
    //     }
    // }

    while catch.load(Ordering::SeqCst) {
        tokio::select! {
            event1 = listening_socket.recv_from(&mut client_incoming) => {

                _ = writeln!(file, "[SENT]");

                let (n, addr) = event1.unwrap();
                println!("\n----------------MESSAGE----------------");
                println!("[PROXY] {} bytes CLIENT -> PROXY", n);

                if drop_chance(args.client_drop) {
                    println!("[PROXY] Client packet stays");

                    if delay_chance(args.client_delay) {
                        
                        let delay = random_delay(args.client_delay_time_min, args.client_delay_time_max);
                        println!("[PROXY] Client packet delayed {} ms", delay);
                        let server_socket = Arc::clone(&server_socket);
                        tokio::spawn(async move {
                            tokio::time::sleep(Duration::from_millis(delay)).await;
                            
                            
                            forward_packet_server(&server_socket, &client_incoming, n).await;
                        });

                    } else {
                        println!("[PROXY] Client packet not delayed");
                        let server_socket_clone = Arc::clone(&server_socket);
                        forward_packet_server(&server_socket_clone, &client_incoming, n).await;
                    }
                    
                } else {
                    println!("[PROXY] Client packet dropped");
                }

                client_addr = Some(addr);
            }

            event2 = server_socket.recv(&mut server_incoming) => {
                let n = event2.unwrap();
                println!("[PROXY] {} bytes PROXY <- SERVER", n);

                if drop_chance(args.server_drop) {
                    println!("[PROXY] Server packet stays");
                    let listening_socket = Arc::clone(&listening_socket);
                    let listening_socket_forward = Arc::clone(&listening_socket);

                    if delay_chance(args.server_delay) {
                        
                        let delay = random_delay(args.server_delay_time_min, args.server_delay_time_max);
                        println!("[PROXY] Server packet delayed {} ms", delay);
                        tokio::spawn(async move {
                            tokio::time::sleep(Duration::from_millis(delay)).await;
                            
                            forward_packet_client(
                                &listening_socket,
                                &client_addr,
                                &server_incoming,
                                n
                            ).await;
                        });
                    } else {
                        println!("[PROXY] Server packet not delayed");
                        forward_packet_client(
                        &listening_socket_forward,
                        &client_addr,
                        &server_incoming,
                        n
                    ).await;
                    }

                    
                    _ = writeln!(file, "[ACK]");
                } else {
                    println!("[PROXY] Server packet dropped");
                }


            }
        }
    }
}
