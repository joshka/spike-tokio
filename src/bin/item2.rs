use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use color_eyre::Result;
use spike_tokio::{fib, NUMBER};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let loop_count = AtomicUsize::new(0);
    let handle = tokio::task::spawn(async { fib(NUMBER) });
    while !handle.is_finished() {
        loop_count.fetch_add(1, Relaxed);
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }
    let result = handle.await?;

    println!(
        "Fibonacci number: {result}, loop_count: {}",
        loop_count.load(Relaxed)
    );
    Ok(())
}
