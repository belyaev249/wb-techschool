use std::thread::{JoinHandle, self};
use std::thread::available_parallelism;
use std::io::stdin;

fn main() {
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let amount_of_cpus: usize = available_parallelism().unwrap().get();
    squares(n, amount_of_cpus);
}

// Для обработки переполнения:
// - Можно пользоваться overflowing_pow() / overflowing_mul()
// - Либо добавить проверку, что n не больше корня из usize::MAX
// - Либо перейти на u128 или BigInt 

fn squares(n: usize, number_of_chunks: usize) {
    let x: Vec<usize> = (1..=n).collect::<Vec<_>>();
    let chunk_size = (n + number_of_chunks - 1) / number_of_chunks;

    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(number_of_chunks);

    for chunk in x.chunks(chunk_size) {
        let chunk = chunk.to_owned();
        let handle = thread::spawn({ 
            move || {
            for c in chunk {
                print!("{} ", c * c);
            }
        }});
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}