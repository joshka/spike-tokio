use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use color_eyre::Result;
use rand::{distributions::Alphanumeric, prelude::*};
use spike_tokio::{fib, NUMBER};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let loop_count = AtomicUsize::new(0);
    let handle = tokio::task::spawn_blocking(|| fib(NUMBER));
    while !handle.is_finished() {
        loop_count.fetch_add(1, Relaxed);
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();
        println!("{}", s);
    }
    let result = handle.await?;

    println!(
        "Fibonacci number: {result}, loop_count: {}",
        loop_count.load(Relaxed)
    );
    Ok(())
}
