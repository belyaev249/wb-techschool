use std::thread;
use std::sync::Arc;
use counters::*;

fn main() {
    let counter1 = LockCounter::new();
    let counter1 = add_one_million(counter1);
    println!("{}", counter1);
    assert_eq!(counter1, 1_000_000);

    let counter2 = AtomicCounter::new();
    let counter2 = add_one_million(counter2);
    println!("{}", counter2);
    assert_eq!(counter2, 1_000_000);
}

fn add_one_million<T: ThreadSafetyCounter + 'static>(counter: T) -> T::Value {
    let mut handles = vec![];
    let counter = Arc::new(counter);

    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 1..=100000 {
                counter.add();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    return counter.get();
}

mod counters {
    use std::sync::{atomic::{AtomicUsize, Ordering}, Mutex};

    pub trait ThreadSafetyCounter: Send + Sync {
        type Value;
        fn add(&self);
        fn get(&self) -> Self::Value;
    }

    // LockCounter
    
    pub struct LockCounter(Mutex<usize>);

    impl LockCounter {
        pub fn new() -> Self {
            LockCounter(Mutex::new(0))
        } 
    }

    impl ThreadSafetyCounter for LockCounter {
        type Value = usize;

        fn add(&self) {
            *self.0.lock().unwrap() += 1;
        }

        fn get(&self) -> Self::Value {
            *self.0.lock().unwrap()
        }
    }

    // AtomicCounter

    pub struct AtomicCounter(AtomicUsize);

    impl AtomicCounter {
        pub fn new() -> Self {
            AtomicCounter(AtomicUsize::new(0))
        } 
    }

    impl ThreadSafetyCounter for AtomicCounter {
        type Value = usize;

        fn add(&self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }

        fn get(&self) -> Self::Value {
            self.0.load(Ordering::Relaxed)
        }
    }
}