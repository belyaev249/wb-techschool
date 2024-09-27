use std::thread::{JoinHandle, self};
use std::thread::available_parallelism;
use std::io::stdin;
use std::sync::mpsc;

fn main() {
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let sum1 = squares_sum_of_sequence(n);
    println!("{sum1}");
    
    let amount_of_cpus: usize = available_parallelism().unwrap().get();
    let sum2 = squares_sum(n, amount_of_cpus);
    println!("{sum2}");

    assert_eq!(sum1, sum2);
}

// Сумму квадратов ряда 1...N можно посчитать по формуле
// n(n + 1)(2n + 1) / 6

fn squares_sum_of_sequence(n: usize) -> usize {
    let n_2 = n * n;
    let n_3 = n_2 * n;
    
    return  (2 * n_3 + 3 * n_2 + n)/6;
}

fn squares_sum(n: usize, number_of_chunks: usize) -> usize {
    let x: Vec<usize> = (1..=n).collect::<Vec<_>>();
    let chunk_size = (n + number_of_chunks - 1) / number_of_chunks;

    let mut sum = 0usize;
    let (sender, receiver) = mpsc::channel::<usize>();
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(number_of_chunks);

    for chunk in x.chunks(chunk_size) {
        let sender = sender.clone();
        let chunk = chunk.to_owned();
        let handle = thread::spawn({ 
            move || {
            for c in chunk {
                sender.send(c * c).unwrap();
            }
        }});
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    while let Ok(recv) = receiver.try_recv() {
        sum += recv;
    }

    return sum;
}