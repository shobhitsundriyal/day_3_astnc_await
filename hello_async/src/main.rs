async fn hello() {
    println!("Hello, async!");
    hello2().await;
    println!("Hello, async part 2!");

    futures::join!(goodbye(), hello2()); // join! macro allows to run multiple futures concurrently, and wait for all of them to complete but does not garantee the order of execution, but in single threaded context it will run in order

    println!("hello ho gay khatam");
}

async fn hello2() {
    println!("Hello, again!");
}

async fn goodbye() {
    println!("Goodbye, async!");
}

fn main() {
    // let future = hello();
    // future.await; //can't call await here, need to create a runtime to run the future

    futures::executor::block_on(hello());
}
