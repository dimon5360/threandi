use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    _nthreads: usize,
    _workers: Vec<Worker>,
    _sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn init(&mut self) {
        let (sender, receiver) = mpsc::channel();

        self._sender = Some(sender);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..self._nthreads {
            self._workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self._sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self._sender.take());

        for worker in &mut self._workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    log::debug!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    log::debug!("Worker {id} disconnected; shutting down.");
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

pub fn new(nthreads: usize) -> ThreadPool {
    ThreadPool {
        _nthreads: nthreads,
        _workers: vec![],
        _sender: None,
    }
}
