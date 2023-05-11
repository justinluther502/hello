mod connection;
mod parameters;
mod threadpool;
use connection::{handle_connection, listen};
use parameters::{CONNS_BEFORE_QUIT, MAX_WORKERS, PORT};
use threadpool::make_threadpool;

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
