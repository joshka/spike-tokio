use std::sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
};

use color_eyre::Result;
use spike_tokio::{fib, NUMBER};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let loop_count = Arc::new(AtomicUsize::new(0));
    let loop_count_clone = loop_count.clone();
    let handle = tokio::task::spawn(async move {
        loop {
            loop_count.fetch_add(1, Relaxed);
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        }
    });
    let result = fib(NUMBER);
    handle.abort();

    println!(
        "Fibonacci number: {result}, loop_count: {}",
        loop_count_clone.load(Relaxed)
    );
    Ok(())
}
