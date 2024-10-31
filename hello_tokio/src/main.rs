use tokio::runtime;

async fn hello() {
    println!("hello, world");
}

fn main() {
    let tokio_runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let tokio_runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap(); //multi-threaded runtime

    tokio_runtime.block_on(hello()); // used tokio_runtime to run the future
}
