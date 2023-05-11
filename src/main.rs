use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream}, 
    process,
};
use hello::{
    TcpConnection,
    ThreadPool,
};
mod parameters;
use parameters::*;

fn main() {
    let listener = TcpListener::bind(PORT).unwrap();
    let pool = ThreadPool::build(MAX_WORKERS).unwrap_or_else(|error| {
        eprintln!("Problem creating thread pool: {error}");
        process::exit(1);
    });

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
