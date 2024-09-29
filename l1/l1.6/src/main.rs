use std::{process, sync::mpsc::{self, Sender}, thread, time::Duration};

const TERMINATE_AFTER_SECONDS: i32 = 5;

fn main() {
    let mut handles = vec![];

    let terminate_handle = thread::spawn(||{
        thread::sleep(Duration::from_secs(TERMINATE_AFTER_SECONDS as u64));
        println!("The program finished working after {} seconds", TERMINATE_AFTER_SECONDS);
        process::exit(0);
    });
    handles.push(terminate_handle);

    let (sender, receiver) = mpsc::channel::<i32>();

    let sender_handle = thread::spawn(move||{
        some_work(&sender);
    });
    handles.push(sender_handle);

    for recv in receiver {
        println!("Received {recv}")
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn some_work(sender: &Sender<i32>) {
    let mut counter = 0;
    loop {
        counter += 1;
        sender.send(counter).unwrap();
        println!("Sent {counter}");
        thread::sleep(Duration::from_millis(200 as u64));
    }
}