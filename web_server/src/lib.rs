use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// holds the threads awaiting to execute code
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}


impl ThreadPool {
    /// Create a new Thread Pool
    /// The size if the number of threads in the pool.
    /// This is also the number of Workers
    ///
    /// #Panics
    /// 'new' will panic if size is 0 or negative
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender)
        }
    }
    // will send a job from the ThreadPool to the Worker instances, which will send the job to its thread
    pub fn execute<F>(&self, f: F)
    where
    // this is how you take in a closure as a parameter
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // drop sender before joining the worker threads
        // no more messages sent
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // call .take() to move thread out of worker
            // we only have a mutable borrow of each worker and
            // .join() takes ownership
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


// item that is sent down the chnnel
type Job = Box<dyn FnOnce() + Send + 'static>;

// picks up code that is to be run and runs it in the Workers thread
// this allows the threads in ThreadPool to WAIT for code that will be sent later
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // this is the thread that 'holds' the code for the pool
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            // explicitly break loop when recv() returns an error
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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