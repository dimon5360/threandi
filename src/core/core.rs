use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use log;

pub trait Interface {
    fn init(&self);
    fn run(&self);
}

struct Core;

impl Interface for Core {
    fn init(&self) {
        log::debug!("init core");
    }

    fn run(&self) {
        log::debug!("run core");

        let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

        for stream in listener.incoming() {
            handle_connection(stream.unwrap());
            println!("Connection established!");
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./static/index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
pub fn new() -> Box<dyn Interface> {
    Box::new(Core {})
}
