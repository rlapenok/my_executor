use std::{future::Future, pin::Pin};

pub type MyFuture = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>;

pub struct Task {
    task: MyFuture,
}
impl Task {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = ()> + Send + Sync + 'static,
    {
        let task = Box::pin(future);
        Self { task }
    }
    pub fn as_mut(&mut self) -> Pin<&mut (dyn Future<Output = ()> + Send + Sync)> {
        self.task.as_mut()
    }
}
