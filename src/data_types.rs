#![allow(dead_code)]
use argh::FromArgs;
use bincode::{Decode, Encode};

/// standard message protocol
#[derive(Encode, Decode)]
pub struct Message {
    pub seq_number: u8,
    pub message: String,
}

#[derive(FromArgs)]
/// Arguments for client program
pub struct ClientArgs {

    /// ip address
    #[argh(option)]
    pub target_ip: String,

    /// port of server
    #[argh(option)]
    pub target_port: u16,

    /// timeout when resending
    #[argh(option, default = "5")]
    pub timeout: u64,

    /// amount of retries for resending
    #[argh(option, default = "3")]
    pub max_retries: u16,
}

#[derive(FromArgs)]
/// Arguments for server program
pub struct ServerArgs {

    /// ip address
    #[argh(option)]
    pub listen_ip: String,

    /// port of server
    #[argh(option)]
    pub listen_port: u16
}

#[derive(FromArgs)]
/// Arguments for proxy server
pub struct ProxyArgs {

    /// forward to this server ip
    #[argh(option)]
    pub listen_ip: String,

    /// server port
    #[argh(option)]
    pub listen_port: String,

    /// forward to this server ip
    #[argh(option)]
    pub target_ip: String,

    /// server port
    #[argh(option)]
    pub target_port: String,

    /// drop chance (%) for packets from client
    #[argh(option, default = "0")]
    pub client_drop: u32,

    /// drop chance (%) for packets from server
    #[argh(option, default = "0")]
    pub server_drop: u32,

    /// delay chance (%) for packets from client
    #[argh(option, default = "0")]
    pub client_delay: u32,

    /// delay chance (%) for packets from server
    #[argh(option, default = "0")]
    pub server_delay: u32,

    /// minimum delay time (ms) for client packets
    #[argh(option, default = "0")]
    pub client_delay_time_min: u32,

    /// maximum delay time (ms) for client packets
    #[argh(option, default = "0")]
    pub client_delay_time_max: u32,

    /// minimum delay time (ms) for server packets
    #[argh(option, default = "0")]
    pub server_delay_time_min: u32,

    /// maximum delay time (ms) for server packets
    #[argh(option, default = "0")]
    pub server_delay_time_max: u32,

}
