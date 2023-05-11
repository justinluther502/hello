use hello::ThreadPool;
use std::process;

mod connection;
mod parameters;
use connection::{handle_connection, listen};
use parameters::{CONNS_BEFORE_QUIT, MAX_WORKERS, PORT};

fn main() {
    let listener = listen(PORT);
    let pool = make_threadpool(MAX_WORKERS);

    for stream in listener.incoming().take(CONNS_BEFORE_QUIT) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.")
}

fn make_threadpool(size: usize) -> ThreadPool {
    ThreadPool::build(size).unwrap_or_else(|error| {
        eprintln!("Problem creating thread pool: {error}");
        process::exit(1);
    })
}
