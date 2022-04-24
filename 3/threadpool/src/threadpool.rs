use std::thread;
use std::sync::mpsc;
use std::sync::{ Mutex, Arc};


pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool{
        assert!(num_threads > 0);

        let mut threads = Vec::with_capacity(num_threads);

        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));

        for task in 0..num_threads {
            threads.push(Worker::new(task, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        //std::thread::sleep(std::time::Duration::from_secs(2));
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //println!("Sending terminate message to all workers.");

        for _ in &self.threads {
            self.sender.send(Message::Terminate).unwrap();
        }

        //println!("Shutting down all workers.");

        for worker in &mut self.threads {
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("This is task: {}", id);

                    job();
                }
                Message::Terminate => {
                    //println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });
        Worker {id, thread: Some(thread)}
    }
}


type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}