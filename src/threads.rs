use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    sender: mpsc::Sender<Job>
} 

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // establish a channel for data to be sent
        // through (sender -> receiver)
        let (sender, receiver) = mpsc::channel();
        // create a thread-safe and shared version of
        // the receiver
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            Worker::new(id, Arc::clone(&receiver));
        }

        ThreadPool {sender}
    }
    pub fn execute<F>(&self, f: F)
    where 
            F: FnOnce() + Send + 'static,
    {
        // allocate the function to be run 
        // in the thread and send it
        // through the channel
        let job: Job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) {
        // create a thread that loops and checks if there
        // is an available job in the channel and if there
        // is, it takes ownership and runs it 
        thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
    }
}
