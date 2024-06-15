#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::Local;
use std::net::TcpListener;

const IP: &str = "127.0.0.1";
const PORT: i16 = 12345;

fn main() {
    println!("server @ http://{IP}:{PORT}");
    let listener = TcpListener::bind(format!("{IP}:{PORT}")).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let now = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("{now} stream");
    }
}
