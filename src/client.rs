mod networking_util;

#[allow(unused_imports)]


use networking_util::{
    check_valid_ip, client_response_handler, client_arg_validation, client_connect, send_message
};
use rand::seq;
// use nix::libc::user;
// use tokio::io::Interest;
// use tokio::io::unix::AsyncFd;
// use tokio::net::TcpListener;
// use serde::Serialize;

use std::{io::{BufRead, BufReader}, net::UdpSocket};
use::std::{process, env};
use std::fs::File;
// use std::os::unix::io::AsRawFd;
// use bincode::{Encode, Decode};

// data structure for sending over
use networking_util::Message;

use crate::networking_util::wait_ack;

#[tokio::main]
async fn main() {

    let mut seq_number: u8 = 0;

    // get user args
    let args: Vec<String> = env::args().collect();

    // verify args
    match client_arg_validation(&args) {
        Ok(()) => {},
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    } 

    //verify ip
    match check_valid_ip(&args[1]) {
        Ok(()) => {},
        Err(e) => {
            println!("Ip address error: {}", e);
            process::exit(1);
        }
    }

    // connect to server
    let socket = match client_connect(&args) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    // match socket.set_nonblocking(true) {
    //     Ok(()) => {},
    //     Err(e) => {
    //         println!("[SERVER] Failed to set socket to non-blocking: {}", e);
    //         process::exit(1);
    //     }
    // };

    loop {
        // listen for keyboard inputs
        let mut input = BufReader::new(File::open("/dev/tty").unwrap());
        println!("Enter Message");

        let mut user_input = String::new();
        input.read_line(&mut user_input).expect("Failed to get input");

        // create the message
        let data: Message = Message {
            seq_number: seq_number,
            message: user_input,
        };

        seq_number += 1;

        println!("Message and sequence number: {} {}fromclient", data.message, data.seq_number);

        // send the message 
        match send_message(&socket, &data) {
            Ok(()) => {},
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }

        // wait for ack, if no response in time resend

        wait_ack(&socket, &data);
        
    }
    
    // ----------------------------------------------------------------------------------------------
    //wait for acks

    // convert socket to listener
    // let listener: std::net::TcpListener = socket.into();
    // let tokio_listener = match TcpListener::from_std(listener) {
    //     Ok(listener) => {listener},
    //     Err(e) => {
    //         eprintln!("[SERVER] Failed to convert socket to a Tokio listener: {}", e);
    //         process::exit(1);
    //     }
    // };

    // match format_send(args.clone(), &socket) {
    //     Ok(()) => {},
    //     Err(e) => {
    //         eprintln!("[CLIENT] Error Sending Data {}", e);
    //         process::exit(1);
    //     }
    // };


        // Send the formatted data
        

        
    // println!("SLEEPING FOR 2 SECONDS");
    // std::thread::sleep(Duration::from_secs(2));

    // Receive the response
    // client_response_handler(&socket);
}
