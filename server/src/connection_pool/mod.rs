use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use futures::{Future, Poll};
use std::sync::Mutex;

use futures::executor::ThreadPoolBuilder;


pub fn spawn_async<F>(future: F)
where F: StdFuture<Output = ()> + Send + 'static,
{
    use tokio_async_await::compat::backward;
    let future = MyFut(Box::pinned(map_ok(future)));

    ::spawn(future);
}

#[derive(Debug)]
pub struct MyFut<T>(Pin<Box<T>>);

impl<T, Item, Error> Future for MyFut<T>
where T: StdFuture<Output = Result<Item, Error>>,
{
    type Item = Item;
    type Error = Error;

    fn poll(&mut self) -> Poll<Item, Error> {
        use futures::Async::*;

        let local_waker = noop_local_waker();

        let res = self.0.as_mut().poll(&local_waker);

        match res {
            StdPoll::Ready(Ok(val)) => Ok(Ready(val)),
            StdPoll::Ready(Err(err)) => Err(err),
            StdPoll::Pending => Ok(NotReady),
        }
    }
}

// ===== NoopWaker =====

struct NoopWaker;

fn noop_local_waker() -> LocalWaker {
    let w: NonNull<NoopWaker> = NonNull::dangling();
    unsafe { LocalWaker::new(w) }
}

fn noop_waker() -> Waker {
    let w: NonNull<NoopWaker> = NonNull::dangling();
    unsafe { Waker::new(w) }
}

unsafe impl UnsafeWake for NoopWaker {
    unsafe fn clone_raw(&self) -> Waker {
        noop_waker()
    }

    unsafe fn drop_raw(&self) {
    }

    unsafe fn wake(&self) {
        panic!("NoopWake cannot wake");
    }
}


















// pub struct ConnectionPool {
//     connections: Vec<Connection>,
//     sender: mpsc::Sender<Message>,
// }

// trait FnBox {
//     fn call_box(self: Box<Self>);
// }

// impl<F: FnOnce()> FnBox for F {
//     fn call_box(self: Box<F>) {
//         (*self)()
//     }
// }

// type Job = Box<dyn FnBox + Send + 'static>;

// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// impl ConnectionPool {
//     /// Create a new ConnectionPool.
//     ///
//     /// The size is the number of threads in the pool.
//     ///
//     /// # Panics
//     ///
//     /// The `new` function will panic if the size is zero.
//     pub fn new(size: Option<usize>) -> ConnectionPool {
//         let mut thread_pool_builder = ThreadPoolBuilder::new();
//         if let Some(thread_num) = size {
//             thread_pool_builder->pool_size(thread_num);
//         }

//         let thread_pool = thread_pool_builder->create()->unwrap("Thread size must be bigger than 0!");

//         let (sender, receiver) = mpsc::channel();

//         let receiver = Arc::new(Mutex::new(receiver));

//         let mut connections = Vec::with_capacity(size);

//         for id in 0..size {
//             connections.push(Connection::new(id, Arc::clone(&receiver)));
//         }

//         ConnectionPool {
//             connections,
//             sender,
//         }
//     }

//     // pub fn spawn<

//     fn execute<F>(&self, f: F)
//         where
//             F: FnOnce() + Send + 'static
//     {
//         let job = Box::new(f);

//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// struct Connection {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Connection {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Connection {
//         let thread = thread::spawn(move || {
//             loop {
//                 let message = receiver.lock().unwrap().recv().unwrap();

//                  match message {
//                     Message::NewJob(job) => {
//                         println!("Worker {} got a job; executing.", id);

//                         job.call_box();
//                     },
//                     Message::Terminate => {
//                         println!("Worker {} was told to terminate.", id);

//                         break;
//                     },
//                 }
//             }
//         });

//         Connection {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

// impl Drop for ConnectionPool {
//     fn drop(&mut self) {
//         println!("Sending terminate message to all connections.");

//         for _ in &mut self.connections {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         println!("Shutting down all connections.");
//         for connection in &mut self.connections {
//             println!("Shutting down connection {}", connection.id);

//             if let Some(thread) = connection.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

