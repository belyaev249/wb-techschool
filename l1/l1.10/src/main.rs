use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const N: usize = 100;

fn main() {
    let mut handles = vec![];
    let (sender1, receiver1) = mpsc::channel::<u64>();
    let (sender2, receiver2) = mpsc::channel::<u64>(); 

    let handle1 = thread::spawn( move || {
        for recv in receiver1 {
            sender2.send(recv * recv).unwrap();
        }
    });
    handles.push(handle1);

    let handle2 = thread::spawn( move || {
        for recv in receiver2 {
            println!("{recv} ");
        }
    });
    handles.push(handle2);

    for x in 1..=N as u64 {
        sender1.send(x).unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    // Закрытие канала для завершения работы очередей
    drop(sender1);

    for handle in handles {
        handle.join().unwrap();
    }
}
