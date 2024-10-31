use tokio::runtime;

async fn hello() {
    println!("hello, world");
}

// ok so if we know that our whole program is async, we can use tokio's simple syntax, eg: in case of a server, whole server is async
// by default, tokio::main will create a runtime for us with multi-threaded executor and max performance settings
// but we can pass our own settings as well, inside the attribute
# [tokio::main(flavor = "current_thread")]
async fn main() {
    // with this we can make the main function async and simply await the async functions
    hello().await;
}

// if there is a small async block in a gaint sync function, we can use executor to run the async block and leave the rest of the function sync
// but if the whole function is async, we can use tokio::main attribute to make the whole function async
// so rust, it give us building blocks and its up to us how we want to use them