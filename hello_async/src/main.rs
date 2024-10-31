async fn hello() {
    println!("Hello, async!");
    hello2().await;
    println!("Hello, async part 2!");

    futures::join!(goodbye(), hello2()); // join! macro allows to run multiple futures concurrently, and wait for all of them to complete but does not garantee the order of execution, but in single threaded context it will run in order

    println!("Double of 2 is {}", double(2).await);

    // something similaer to promises.all in js
    let futures_vec = vec![double(3), double(4), double(5)];
    let results = futures::future::join_all(futures_vec).await;
    println!("{results:?}");
}

async fn hello2() {
    println!("Hello, again!");
}

async fn goodbye() {
    println!("Goodbye, async!");
}

async fn double (x: i32) -> i32 {
    return x*2;
}

fn main() {
    // let future = hello();
    // future.await; //can't call await here, need to create a runtime to run the future

    futures::executor::block_on(hello()); // async context needs a runtime to run
}

// we can normally call sync functions inside async functions, but we can't call async functions inside sync functions without a runtime i.e. using block_on