#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::Local;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const IP: &str = "127.0.0.1";
const PORT: i16 = 12345;

fn main() {
    println!("server @ http://{IP}:{PORT}");
    let listener = TcpListener::bind(format!("{IP}:{PORT}")).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("{now} stream");
        route(stream);
    }
}

fn route(stream: TcpStream) {
    let buf_reader = BufReader::new(stream);
    let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).collect();
    println!("{http_request:#?}");
}
