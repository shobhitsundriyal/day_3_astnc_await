use tokio::runtime;

async fn hello() {
    println!("hello, world");
}

// ok so if we know that our whole program is async, we can use tokio's simple syntax, eg: in case of a server, whole server is async
# [tokio::main]
async fn main() {
    // with this we can make the main function async and simply await the async functions
    hello().await;
}
