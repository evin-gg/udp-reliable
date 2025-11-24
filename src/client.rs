mod networking_util;

#[allow(unused_imports)]


use networking_util::{
    check_valid_ip, client_response_handler, client_arg_validation, client_connect, send_message
};

use std::{io::{BufRead, BufReader}, net::UdpSocket};
use::std::{process, env};
use std::fs::File;

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

    let std_socket: UdpSocket = socket.into();

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

        

        println!("[CLIENT] Message and seq number: {} {}", data.message.trim_end(), data.seq_number);

        // send the message 
        match send_message(&std_socket, &data) {
            Ok(()) => {},
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        }

        // wait for ack, if no response in time resend
        match wait_ack(&std_socket, &data, args[3].parse().unwrap(), args[4].parse().unwrap()) {
            Ok(()) => {},
            Err(e) => {
                println!("{}", e);
            }
        };

        seq_number += 1;
        
    }
}
