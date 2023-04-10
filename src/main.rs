mod executor;
use executor::executor::Executor;

async fn hello_world() {
    println!("Hello World");
}

async fn hello() {
    println!("Hello");
}

fn main() {
    let mut executor = Executor::new();
    executor.spawn(hello_world()).spawn(hello());
    executor.run();
}
