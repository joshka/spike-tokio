use std::sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
};

use color_eyre::Result;
use rand::{distributions::Alphanumeric, prelude::*};
use spike_tokio::{fib, NUMBER};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let loop_count = Arc::new(AtomicUsize::new(0));
    let loop_count_clone = loop_count.clone();
    let handle = tokio::task::spawn_blocking(move || loop {
        loop_count.fetch_add(1, Relaxed);
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1000)
            .map(char::from)
            .collect();
        println!("{}", s);
    });
    let result = fib(NUMBER);
    handle.abort();

    println!(
        "Fibonacci number: {result}, loop_count: {}",
        loop_count_clone.load(Relaxed)
    );
    Ok(())
}
