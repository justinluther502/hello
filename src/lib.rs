use std::{
    fmt, fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
    vec::Vec,
};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool {
                _workers: workers,
                sender: Some(sender),
            })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self._workers {
            println!("Shutting down worker {}", worker._id);
            if let Some(thread) = worker._thread.take() {
                thread.join().unwrap()
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    _thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker {
            _id: id,
            _thread: Some(thread),
        }
    }
}

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreadPool size should be a positive integer.")
    }
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
