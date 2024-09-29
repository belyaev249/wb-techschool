use std::{thread, sync::mpsc, time::Duration};
use tokio_util::sync::CancellationToken;

const TERMINATE_AFTER_SECONDS: u32 = 4;

#[tokio::main]
async fn main() {
    // std::thread
    let (sender, receiver) = mpsc::channel::<()>();

    let thread_handle = thread::spawn(move ||{
        loop {
            match sender.send(()) {
                Ok(_) => println!("Send from std::thread"),
                Err(_) => break
            }
            thread::sleep(Duration::from_millis(200));
        }
        println!("std::thread closed")
    });

    // tokio::task
    let token = CancellationToken::new();

    let task_token = token.clone();
    let task_handle = tokio::spawn(async move {
        loop {
            match task_token.is_cancelled() {
                false => println!("Send from tokio::task"),
                true => break
            }
            thread::sleep(Duration::from_millis(200));
        }
        println!("tokio::task closed")
    });
    
    // close std::thread and tokio::task
    thread::sleep(Duration::from_secs(TERMINATE_AFTER_SECONDS as u64));
    drop(receiver);
    token.cancel();

    thread_handle.join().unwrap();
    task_handle.await.unwrap();
    
}
