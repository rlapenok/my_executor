use std::task::Wake;

pub struct MyWaker {
    clouser: Box<dyn Fn() + Send + Sync + 'static>,
}

impl MyWaker {
    pub fn new<C: Fn() + 'static + Send + Sync>(clouser: C) -> Self {
        let clouser = Box::new(clouser);
        Self { clouser }
    }
}

impl Wake for MyWaker {
    fn wake(self: std::sync::Arc<Self>) {
        let clouser = &*self.clone().clouser;
        clouser()
    }
}
