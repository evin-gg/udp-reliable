#![allow(dead_code)]
use argh::FromArgs;
use bincode::{Decode, Encode};


#[derive(Encode, Decode)]
pub struct Message {
    pub seq_number: u8,
    pub message: String,
}

#[derive(FromArgs)]
/// Arguments
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
/// Arguments
pub struct ServerArgs {

    /// ip address
    #[argh(option)]
    pub target_ip: String,

    /// port of server
    #[argh(option)]
    pub target_port: u16
}