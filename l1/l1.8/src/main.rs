use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Mutex, Arc};
use std::thread::{self, available_parallelism};
use dashmap::DashMap;

const MAP_CAPACITY: usize = 10;

fn main() {
    let mut handles = vec![];
    let number_of_cpus = available_parallelism().unwrap().get();

    let hashmap = Arc::new(Mutex::new(HashMap::<usize, usize>::with_capacity(MAP_CAPACITY)));
    let dashmap = Arc::new(DashMap::<usize, usize>::with_capacity(MAP_CAPACITY));

    for _ in 1..=number_of_cpus {
        let hashmap = Arc::clone(&hashmap);
        let handle = thread::spawn( move || {
            for _ in 1..=10 {
                for i in 1..=MAP_CAPACITY {
                    let mut guard = hashmap.lock().unwrap();
                    if let Some(value) = guard.get_mut(&i) {
                        *value += 1;
                    } else {
                        guard.insert(i, 1);
                    }
                }
            }
        });
        handles.push(handle);
    }

    for _ in 1..=number_of_cpus {
        let dashmap = Arc::clone(&dashmap);
        let handle = thread::spawn(move || {
            for _ in 1..=10 {
                for i in 1..=MAP_CAPACITY {
                    if let Some(mut value) = dashmap.get_mut(&i) {
                        *value += 1;
                    } else {
                        dashmap.insert(i, 1);
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    let values1: Vec<usize> = hashmap.lock().unwrap().values().map(|v|v.to_owned()).collect();
    let values2: Vec<usize> = dashmap.iter().map(|v|v.deref().to_owned()).collect();
    assert_eq!(values1, values2);
}
