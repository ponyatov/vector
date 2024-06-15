#![allow(dead_code)]
#![allow(unused)]
#![allow(non_upper_case_globals)]

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
        handle(stream);
    }
}

fn handle(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let req: &str = http_request[0].split(' ').collect::<Vec<_>>()[1];
    route(stream, req);
}

macro_rules! file {
    ($n:ident,$s:literal) => {
        static $n: &'static [u8] = include_bytes!($s);
    };
}

file!(index_html, "../static/index.html");
file!(logo_png, "../doc/logo.png");
file!(css_css, "../static/css.css");
file!(jquery_js, "../static/cdn/jquery.js");

macro_rules! headline {
    ($n:ident,$s:literal) => {
        static $n: &'static [u8] = concat!($s, "\r\n").as_bytes();
    };
}

headline!(OK, "HTTP/1.1 200 OK");
headline!(ERR404, "HTTP/1.1 404 NotFound");
headline!(CR, "");

macro_rules! mime {
    ($n:ident,$s:literal,$e:literal) => {
        static $n: &'static [u8] =
            concat!("Content-Type: ", $s, "; charset=", $e, "\r\n").as_bytes();
    };
    ($n:ident,$s:literal) => {
        static $n: &'static [u8] = concat!("Content-Type: ", $s, "\r\n").as_bytes();
    };
}

mime!(text_plain, "text/plain", "utf-8");
mime!(text_html, "text/html", "utf-8");
mime!(text_css, "text/css", "utf-8");
mime!(app_js, "application/javascript", "utf-8");
mime!(image_png, "image/png");

fn route(mut stream: TcpStream, req: &str) {
    println!("{req}");

    macro_rules! send {
        ($($a:ident),+) => {
            $( stream.write_all($a); )*
        };
    }

    match req.trim() {
        "/" => {
            stream.write_all(OK);
            stream.write_all(text_html);
            stream.write_all(CR);
            stream.write_all(index_html);
        }
        "/favicon.ico" | "/logo.png" => {
            stream.write_all(OK);
            stream.write_all(image_png);
            stream.write_all(CR);
            stream.write_all(logo_png);
        }
        "/css.css" => {
            send!(OK);
            send!(text_css, CR, css_css);
            // stream.write_all(OK);
            // stream.write_all(text_css);
            // stream.write_all(CR);
            // stream.write_all(css_css);
        }
        "/cdn/jquery.js" => {
            stream.write_all(OK);
            stream.write_all(app_js);
            stream.write_all(CR);
            stream.write_all(jquery_js);
        }
        _ => {
            stream.write_all(ERR404);
            stream.write_all(text_plain);
            stream.write_all(CR);
            stream.write_all(format!("ERR404: {req}").as_bytes());
        }
    }
}
