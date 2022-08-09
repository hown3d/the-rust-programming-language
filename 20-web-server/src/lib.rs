use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle, Thread},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, reciever) = mpsc::channel();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut threads = Vec::with_capacity(size);
        for i in 0..size {
            threads.push(Worker::new(i, Arc::clone(&reciever)))
        }

        ThreadPool {
            workers: threads,
            sender,
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Message::NewJob(Box::new(f));
        self.sender.send(job).unwrap();
    }
}

// make sure to provide a graceful shutdon
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in self.workers.iter() {
            println!("Shutting down worker {}", worker.id);
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = reciever.lock().unwrap().recv().unwrap();
            match msg {
                Message::NewJob(job) => {
                    println!("Worker {} got job; executing", id);
                    job()
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
