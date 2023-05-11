use hello::{TcpConnection, ThreadPool};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    process,
};
mod parameters;
use parameters::*;

fn main() {
    let listener = TcpListener::bind(PORT).unwrap();
    let pool = make_threadpool(MAX_WORKERS);

    for stream in listener.incoming().take(CONNS_BEFORE_QUIT) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.")
}

fn handle_connection(mut stream: TcpStream) {
    let connection = TcpConnection::new(&stream);
    let response = connection.response();
    stream.write_all(response.as_bytes()).unwrap();
}

fn make_threadpool(size: usize) -> ThreadPool {
    ThreadPool::build(size).unwrap_or_else(|error| {
        eprintln!("Problem creating thread pool: {error}");
        process::exit(1);
    })
}
