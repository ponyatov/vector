#![allow(dead_code)]
#![allow(unused)]
#![allow(non_upper_case_globals)]

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    os::unix::{io::AsRawFd, process::CommandExt},
    path::Path,
    process::Command,
    thread, time,
};

use notify::{RecursiveMode, Watcher};

const IP: &str = "127.0.0.1";
const PORT: i16 = 12345;

fn main() {
    let argv0 = std::env::current_exe().unwrap();
    let argv_0 = argv0.display();
    println!("{argv_0} @ http://{IP}:{PORT}");

    let listener = TcpListener::bind(format!("{IP}:{PORT}")).unwrap();
    let listener_sock = listener.as_raw_fd();

    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            unsafe {
                libc::close(listener_sock);
                // libc::shutdown(listener_fd, libc::SHUT_RD);
            };
            Command::new("proc/self/exe").exec();
            std::process::exit(0)
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();

    watcher
        .watch(Path::new(&argv0), RecursiveMode::NonRecursive)
        .unwrap();

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
        ($a:ident,$b:ident) => {
            send!(OK,$a,CR,$b)
        };
        ($($a:ident),+) => {
            $( stream.write_all($a); )*
        };
        ($($a:expr),+) => {
            $( stream.write_all($a); )*
        };
    }

    match req.trim() {
        "/" => {
            send!(text_html, index_html);
        }
        "/favicon.ico" | "/logo.png" => {
            send!(image_png, logo_png);
        }
        "/css.css" => {
            send!(text_css, css_css);
        }
        "/cdn/jquery.js" => {
            send!(app_js, jquery_js);
        }

        _ => {
            send!(ERR404, text_html, CR);
            send!("<link href=/css.css rel=stylesheet type=text/css>".as_bytes());
            send!(format!("<h1 class=error>Not Found: {req}").as_bytes());
        }
    }
}
