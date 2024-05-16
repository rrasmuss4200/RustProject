use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// holds the threads awaiting to execute code
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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

        ThreadPool { workers, sender }
    }
    // will send a job from the ThreadPool to the Worker instances, which will send the job to its thread
    pub fn execute<F>(&self, f: F)
    where
    // this is how you take in a closure as a parameter
        F: FnOnce() + Send + 'static,
        {

        }
}

// item that is sent down the chnnel
struct Job;

// picks up code that is to be run and runs it in the Workers thread
// this allows the threads in ThreadPool to WAIT for code that will be sent later
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // this is the thread that 'holds' the code for the pool
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker {
            id,
            thread
        }
    }
}