use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration, process,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(10).unwrap_or_else(|error| {
        eprintln!("Problem creating thread pool: {error}");
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = TcpRequest::new(&stream);
    let status_line = request.response_status_line();
    let filepath = request.response_html_filestring();
    let contents = fs::read_to_string(filepath).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    stream.write_all(response.as_bytes()).unwrap();    
}

struct TcpRequest {
    _method: String,
    route: String,
    _version: String,
}

impl TcpRequest {
    pub fn new(stream: &TcpStream) -> TcpRequest {
        let buf_reader = BufReader::new(stream);
        let first_request_line = buf_reader.lines().next().unwrap().unwrap();
        let mut tcp_args = first_request_line.split_whitespace();
        let method = tcp_args.next().unwrap().to_owned();
        let route = tcp_args.next().unwrap().to_owned();
        let version = tcp_args.next().unwrap().to_owned();
        TcpRequest { _method: method, route, _version: version }
    }

    pub fn response_status_line(&self) -> String {
        match &self.route[..] {
            "/" => String::from("HTTP/1.1 200 OK"),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                String::from("HTTP/1.1 200 OK")
            }
            _ => String::from("HTTP/1.1 404 NOT FOUND"),
        }
    }

    pub fn response_html_filestring(&self) -> String {
        match &self.route[..] {
            "/" | "/sleep" => String::from("pages/hello.html"),
            _ => String::from("pages/404.html"),
        }
    }
}
