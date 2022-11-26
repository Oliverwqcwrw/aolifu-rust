use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;


struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

trait FnBox{
    fn call_box(self: Box<Self>);
}


type Job = Box<dyn FnBox + Send + 'static>;

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job;executing", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("worker {} is told to terminate ", id);
                    break
                },
        }
        });
        Worker{
            id,
            thread: Some(thread),
        }
    }
}

enum Message{
    NewJob(Job),
    Terminate,
}


pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of the threads in the pool
    ///
    /// Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let (sender,receiver) = mpsc::channel();
        let receiver  = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{
            workers,sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("shutting down all workers");
        for worker in &mut self.workers{
            println!("Shutting down worker{}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


pub fn server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let mut stream = stream.unwrap();
        println!("connection established");

        pool.execute(|| {
            handle_connection(stream);
        });


    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line,file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "static/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "static/404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();
    let response = format!("{}{}",status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}