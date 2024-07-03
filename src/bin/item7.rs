use std::{
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc,
    },
    time::Duration,
};

use color_eyre::Result;
use spike_tokio::{fib, NUMBER};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let loop_count = Arc::new(AtomicUsize::new(0));
    let loop_count_clone = loop_count.clone();
    let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
    let handle = tokio::task::spawn_blocking(move || loop {
        loop_count.fetch_add(1, Relaxed);
        std::thread::sleep(Duration::from_millis(20));
        if rx.try_recv().is_ok() {
            break;
        }
    });
    let result = fib(NUMBER);
    tx.send(()).unwrap();
    handle.await?;
    // handle.abort();
    println!(
        "Fibonacci number: {result}, loop_count: {}",
        loop_count_clone.load(Relaxed)
    );
    Ok(())
}
