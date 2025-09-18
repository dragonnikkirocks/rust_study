use tokio::time::{sleep, Duration};

//shows how futures in rust is not started until explicilty called by runtime as as Tokio
// Key points:
// Lazy: nothing happens until you .await (or poll explicitly).
// Built into the language (async fn returns an impl Future).
// Driven by an executor (e.g., Tokio, async-std).
// Zero-cost state machine — no hidden threads.

async fn demo_task(name: &str) {
    println!("Task {name} started");
    sleep(Duration::from_secs(1)).await;
    println!("Task {name} finished");
}

#[tokio::main]
async fn main() {
    // Step 1: Create the future, but don’t .await yet
    let fut_a = demo_task("A");
    let fut_b = demo_task("B");

    println!("Futures created, but not running yet!");

    // Step 2: Now we await them together
    futures::join!(fut_a, fut_b);

    println!("All done!");
}
