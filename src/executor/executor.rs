use super::my_waker::*;
use super::task::Task;
use std::{
    collections::VecDeque,
    future::Future,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Waker},
};

pub struct Executor {
    queue: VecDeque<Arc<Mutex<Task>>>,
    sender: SyncSender<Arc<Mutex<Task>>>,
    receiver: Receiver<Arc<Mutex<Task>>>,
}

impl Executor {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel(1000);
        Self {
            queue: VecDeque::new(),
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn spawn<F>(&mut self, future: F) -> &mut Self
    where
        F: Future<Output = ()> + Send + Sync + 'static,
    {
        let task = Task::new(future);
        self.queue.push_back(Arc::new(Mutex::new(task)));
        self
    }
    pub fn run(&mut self) {
        loop {
            while let Some(task) = self.queue.pop_front() {
                let waker = {
                    let task_for_waker = task.clone();
                    let sender = self.sender.clone();
                    let my_waker = Arc::new(MyWaker::new(move || {
                        sender.send(task_for_waker.clone()).unwrap();
                    }));
                    Waker::from(my_waker)
                };
                let mut cx = Context::from_waker(&waker);
                match task.lock().unwrap().as_mut().poll(&mut cx) {
                    std::task::Poll::Ready(_result) => {
                        eprintln!("Task ready");
                    }
                    std::task::Poll::Pending => {
                        waker.wake();
                        let task = self.receiver.recv().unwrap();
                        self.queue.push_back(task);
                    }
                }
            }
        }
    }
}
