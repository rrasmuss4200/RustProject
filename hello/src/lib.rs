use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new Thread Pool
    /// The size if the number of threads in the pool
    ///
    /// #Panics
    /// 'new' will panic if size is 0 or negative
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {

        }

        ThreadPool {threads}
    }

    pub fn execute<F>(&self, f: F)
    where
    // this is how you take in a closure as a parameter
        F: FnOnce() + Send + 'static,
        {

        }
}