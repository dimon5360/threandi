use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use log;
use std::time::Duration;

struct Core {
    _pool: super::pool::ThreadPool,
    _listener: Option<TcpListener>,
}

impl super::provider::CoreInterface for Core {
    fn init(&mut self, host: &str) {
        log::debug!("init core");

        self._listener = Some(TcpListener::bind(host).unwrap());
    }

    fn run(&mut self) {
        match &self._listener {
            Some(listener) => {
                for stream in listener.incoming() {
                    let stream = stream.unwrap();

                    self._pool.execute(|| {
                        handle_connection(stream);
                    })
                }
            }
            None => {
                panic!("listener isn't initialized");
            }
        }

        log::debug!("run core");
    }
}

fn handle_connection(mut stream: TcpStream) {
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

pub fn new(pool: super::pool::ThreadPool) -> Box<dyn super::provider::CoreInterface> {
    Box::new(Core {
        _pool: pool,
        _listener: None,
    })
}
