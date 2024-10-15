fn main() {
    lf::process();
}

mod lf {
    use clap::{Parser, arg};
    use serde::Serialize;
    use std::fs::File;
    use std::io::{Read, stdin};
    use std::path::PathBuf;
    use std::thread;
    use std::time::Instant;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[derive(Serialize)]
    struct Info {
        elapsed: std::time::Duration,
        result: HashMap<char, usize>
    }

    #[derive(Parser)]
    struct Args {
        #[arg(default_value_t=1, short='t', long, help="")]
        threads_number: usize,
        file_path: Option<PathBuf>
    }

    pub fn process() {
        let args = Args::parse();

        if let Some(file_path) = args.file_path {
            read_from_file(&file_path, args.threads_number);
        } else {
            read_from_stdin(args.threads_number);
        }
    }

    fn read_from_file(file_path: &PathBuf, threads_number: usize) {
        let mut file = File::open(file_path).unwrap();
        let mut input_str = String::new();
        file.read_to_string(&mut input_str).unwrap();
        let info = read_str_without_lock(&input_str.chars().collect(), threads_number);
        let j = serde_json::to_string_pretty(&info).unwrap();
        println!("{j:#}");
    }

    fn read_from_stdin(threads_number: usize) {
        println!("Начните вводить произвольные строки");
        loop {
            let mut input_str = String::new();
            stdin().read_line(&mut input_str).unwrap();
            let info = read_str_without_lock(&input_str.chars().collect(), threads_number);
            let j = serde_json::to_string_pretty(&info).unwrap();
            println!("{j:#}");
        }
    }

    // Подход без mutex:
    // На каждый поток свой массив счетчиков
    // Потоки не ждут пока освободится lock
    // Массивы вместо словаря, не тратится время на хэш функцию
    fn read_str_without_lock(chars: &Vec<char>, threads_number: usize) -> Info {        
        thread::scope(|s| {
            let d: Instant = Instant::now();
            let chunk_size: usize = (chars.len() + threads_number - 1) / threads_number;
            let chunks = chars.chunks(chunk_size);
            
            let mut handles = vec![];
            for chars_chunk in chunks {
                let handle = s.spawn(move || {
                    let mut letters = [0usize; 26];
                    for ch in chars_chunk {
                        if ch.is_ascii_alphabetic() {
                            let mut x = [0];
                            ch.encode_utf8(&mut x);
                            let mut x: usize = x[0] as usize;
                            if x < 91 {
                                x -= 65;
                            } else {
                                x -= 97;
                            }
                            letters[x] += 1;
                        }
                    }
                    return letters;
                });
                handles.push(handle);
            } 

            let mut result = "qwertyuiopasdfghjklzxcvbnm".chars().map(|ch|(ch, 0usize)).collect::<HashMap<_, _>>();
            for handle in handles {
                let letters = handle.join().unwrap();
                for i in 0..=letters.len()-1 {
                    let c = letters[i];
                    *result.get_mut(&char::from_u32(i as u32 + 97).unwrap()).unwrap() += c;
                }
            }
            
            return Info { elapsed: d.elapsed(), result };
        })
    }

    // Подход с Arc, Mutex и общим словарем
    fn read_str_with_lock(chars: &Vec<char>, threads_number: usize) -> Info {
        thread::scope(|s| {
            let d: Instant = Instant::now();
            let chunk_size: usize = (chars.len() + threads_number - 1) / threads_number;
            let chunks = chars.chunks(chunk_size);

            let mut handles = vec![];
            let letters = "qwertyuiopasdfghjklzxcvbnm".chars().map(|ch|(ch, 0usize)).collect::<HashMap<_, _>>();
            let letters = Arc::new(Mutex::new(letters));

            for chars_chunk in chunks {
                let letters = Arc::clone(&letters);
                let handle = s.spawn(move ||{
                    for ch in chars_chunk {
                        if ch.is_ascii_alphabetic() {
                            let mut lock = letters.lock().unwrap();
                            *lock.get_mut(&ch.to_ascii_lowercase()).unwrap() += 1;
                        }
                    }
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            return Info { elapsed: d.elapsed(), result: letters.lock().unwrap().clone() };
        })
    }
}