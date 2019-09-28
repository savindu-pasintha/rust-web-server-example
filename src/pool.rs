use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

type Job = Box<dyn FnBox + Send + 'static>;
type JobReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;
type Thread = thread::JoinHandle<()>;

pub struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {

    pub fn new(size: usize) -> WorkerPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        return WorkerPool{workers, sender};
    }

    pub fn run<Func>(&self, func: Func) where Func: FnOnce() + Send + 'static {
        let job = Box::new(func);
        self.sender.send(job).unwrap();
    }
}

pub struct Worker {
    id: usize,
    thread: Thread,
}

impl Worker {

    pub fn new(id: usize, receiver: JobReceiver) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                job.run();
            }
        });
        return Worker{id, thread};
    }
}

pub trait FnBox {
    fn run(self: Box<Self>);
}

impl<Func: FnOnce()> FnBox for Func {

    fn run(self: Box<Func>) {
        return (*self)();
    }
}
