extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};

// a server in tokio_proto is made up of three distinct parts
// a transport ...   implemented using the framed helper
// a protocol specification ..   codex, basic info, multiplexed?, streaming?
// a service .. how to produce a response give a request .. async function

pub struct LineCodec;


fn main() {
    println!("Hello, world!");
}
