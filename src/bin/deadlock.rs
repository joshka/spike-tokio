use std::{thread, time::Duration};

use color_eyre::Result;
use tokio::join;

fn main() -> Result<()> {
    color_eyre::install()?;
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?
        .block_on(run())
}

async fn run() -> Result<()> {
    let (tx, mut rx) = tokio::sync::oneshot::channel();
    let task1 = tokio::task::spawn(async move {
        println!("Task 1 waiting for message");
        loop {
            thread::sleep(Duration::from_millis(1));
            if let Ok(()) = rx.try_recv() {
                break;
            }
        }
        unreachable!("Task 1 received message");
    });
    let task2 = tokio::task::spawn(async move {
        println!("Task 2 sleeping for 20ms");
        tokio::time::sleep(Duration::from_millis(20)).await; // ensure the other task runs first
        unreachable!("Task 2 sending message");
        tx.send(()).unwrap();
        unreachable!("Task 2 sent message");
    });
    let _ = join!(task1, task2);
    Ok(())
}
