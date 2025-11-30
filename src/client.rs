#[allow(unused_imports)]

mod util;
mod data_types;

// keyboard listening and udp sockets
use std::{io::{Write}, net::UdpSocket};
use std::fs::File;

// process exit
use::std::{process};

// data types
use data_types::{ClientArgs};

// functions and utility
use crate::util::client_util::*;
use crate::util::networking_util::check_valid_ip;

#[tokio::main]
async fn main() {

    let mut seq_number: u8 = 0;
    let args: ClientArgs = argh::from_env();

    //verify ip
    match check_valid_ip(&args.target_ip) {
        Ok(()) => {},
        Err(e) => {
            println!("Ip address error: {}", e);
            process::exit(1);
        }
    }

    // log
    let mut file = match File::create("./loggers/logs/client.log") {
        Ok(f) => f,
        Err(e) => {
            println!("[CLIENT] Could not create log file: {}", e);
            process::exit(1);
        }
    };

    let fileclone = match file.try_clone() {
        Ok(f) => {f},
        Err(e) => {
            println!("[CLIENT] Error cloning file: {}", e);
            process::exit(1);
        }
    };

    // connect to server
    let socket = match client_connect(&args) {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let std_socket: UdpSocket = socket.into();
    std::process::Command::new("clear").status().unwrap();
    loop {
        // listen for keyboard inputs
        let user_input = listen_keyboard();

        if user_input == "\n" {
            std::process::Command::new("clear").status().unwrap();
            continue;
        }

        // create the message
        let data = create_message(seq_number, user_input);

        std::process::Command::new("clear").status().unwrap();
        println!("[CLIENT]");
        println!("Message: {}", data.message.trim_end());
        println!("Sequence: {}\n", data.seq_number);

        // send the message 
        match send_message(&std_socket, &data) {
            Ok(()) => {},
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }
        _ = writeln!(file, "[SENT]");

        // wait for ack, if no response in time resend
        match wait_ack(&std_socket, &data, args.timeout, args.max_retries.into(), &fileclone) {
            Ok(()) => {_ = writeln!(file, "[ACK]")},
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };

        seq_number += 1;
    }
}
