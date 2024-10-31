use tokio::runtime;

async fn hello() {
    println!("hello, world");
}

// ok so if we know that our whole program is async, we can use tokio's simple syntax, eg: in case of a server, whole server is async
// by default, tokio::main will create a runtime for us with multi-threaded executor and max performance settings
// but we can pass our own settings as well
# [tokio::main]
async fn main() {
    // with this we can make the main function async and simply await the async functions
    hello().await;
}
