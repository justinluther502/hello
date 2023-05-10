use hello::{
    TcpConnection,
    ThreadPool,
};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream}, 
    process,
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
    let connection = TcpConnection::new(&stream);
    let response = connection.response();
    stream.write_all(response.as_bytes()).unwrap();    
}
