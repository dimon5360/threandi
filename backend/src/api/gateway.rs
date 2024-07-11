use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use super::endpoint::Endpoint;

/// TODO: work with endpoints
/// endpoints list getting from core service

pub trait Interface: Send + Sync {
    fn attach(&mut self, endpoint: Endpoint);
    fn handle_connection(&self, stream: TcpStream);
}

pub struct Gateway {
    _endpoints: Vec<Endpoint>,
}

pub fn new() -> Box<dyn Interface> {
    Box::new(Gateway { _endpoints: vec![] })
}

impl Interface for Gateway {
    fn attach(&mut self, endpoint: Endpoint) {
        self._endpoints.push(endpoint);
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./static/index.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "./static/index.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "./static/404.html"),
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
