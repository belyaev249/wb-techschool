use std::{io::stdin, thread};

fn main() {
    println!("Введите количество воркеров:");
    let mut number_of_workers = String::new();
    stdin().read_line(&mut number_of_workers).unwrap();
    let number_of_workers: u32 = number_of_workers.trim().parse().unwrap();

    let (sender, mut receiver) = mpmc::channel::<String>(number_of_workers as usize);

    for i in 1..=number_of_workers {
        let receiver = receiver.get_recv();
        thread::spawn({move || {
            loop {
                for value in &receiver {
                    print!("from worker {i}: {value}");
                }
            }
        }});
    }

    println!("Начните вводить произвольные данные:");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        sender.send(input);
    }
}

mod mpmc {
    use std::sync::mpsc;

    pub struct Sender<T> {
        senders: Vec<mpsc::Sender<T>>
    }

    impl<T> self::Sender<T> {
        pub fn send(&self, t: T) where T: Clone {
            for sender in &self.senders {
                sender.send(t.clone()).unwrap();
            }
        }
    }

    pub struct Receiver<T> {
        receivers: Vec<mpsc::Receiver<T>>
    }

    impl<T> self::Receiver<T> {
        pub fn get_recv(&mut self) -> mpsc::Receiver<T> {
            self.receivers.pop().unwrap()
        }
    }

    pub fn channel<T>(size: usize) -> (self::Sender<T>, self::Receiver<T>) {
        let mut senders: Vec<mpsc::Sender<T>> = Vec::with_capacity(size);
        let mut receivers: Vec<mpsc::Receiver<T>> = Vec::with_capacity(size);
        for _ in 1..=size {
            let (sender, receiver) = mpsc::channel::<T>();
            senders.push(sender);
            receivers.push(receiver);
        }
        return (Sender { senders }, Receiver { receivers });
    }

}