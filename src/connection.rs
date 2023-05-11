use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    process, thread,
    time::Duration,
};

pub fn handle_connection(mut stream: TcpStream) {
    let connection = TcpConnection::new(&stream);
    let response = connection.response();
    stream.write_all(response.as_bytes()).unwrap_or_else(|error| {
        eprintln!("Problem writing response to TCP stream: {error}.");
        process::exit(1);
    });
}

pub fn listen(port: &str) -> TcpListener {
    TcpListener::bind(port).unwrap_or_else(|err| {
        eprintln!("Problem binding to port: {err}.");
        process::exit(1);
    })
}

pub struct TcpConnection {
    _method: String,
    route: String,
    _version: String,
}

impl TcpConnection {
    pub fn new(stream: &TcpStream) -> TcpConnection {
        let buf_reader = BufReader::new(stream);
        let first_request_line = buf_reader.lines().next().unwrap().unwrap();
        let mut tcp_args = first_request_line.split_whitespace();
        let method = tcp_args.next().unwrap().to_owned();
        let route = tcp_args.next().unwrap().to_owned();
        let version = tcp_args.next().unwrap().to_owned();
        TcpConnection {
            _method: method,
            route,
            _version: version,
        }
    }

    pub fn response(&self) -> String {
        let status_line = self.response_status_line();
        let filepath = self.response_html_filestring();
        let contents = fs::read_to_string(filepath).unwrap();
        let length = contents.len();
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
    }

    fn response_status_line(&self) -> String {
        match &self.route[..] {
            "/" => String::from("HTTP/1.1 200 OK"),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                String::from("HTTP/1.1 200 OK")
            }
            _ => String::from("HTTP/1.1 404 NOT FOUND"),
        }
    }

    fn response_html_filestring(&self) -> String {
        match &self.route[..] {
            "/" | "/sleep" => String::from("pages/hello.html"),
            _ => String::from("pages/404.html"),
        }
    }
}
