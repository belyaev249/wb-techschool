fn main() {
    wc::process().unwrap();
}

// Для запуска:
// cargo run -- [params] [file_name]
// Параметры:
// -c подсчет символов (по умолчанию)
// -l подсчет строк
// -w подсчет слов

// Пример: cargo run -- -c file.txt  

mod wc {
    use std::collections::HashSet;
    use std::env;
    use std::fmt::Debug;
    use std::io::Read;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::thread;
    use std::slice::Chunks;
    use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

    use unicode_segmentation::UnicodeSegmentation;

    pub enum Error {
        InvalidFileExtension,
        Unknown(String),
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let msg = match self {
                Error::InvalidFileExtension => "The input file extension must be .txt",
                Error::Unknown(msg) => &msg
            };
            write!(f, "{msg}")?;
            Ok(())
        }
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Error {
            Error::Unknown(format!("{}", value.kind()))
        }
    }

    impl From<std::convert::Infallible> for Error {
        fn from(value: std::convert::Infallible) -> Error {
            Error::Unknown(String::from("Infallible"))
        }
    }

    #[derive(PartialEq, Eq, Hash, Debug)]
    enum Parameter {
        Chars, Lines, Words
    }

    struct Options {
        file_name: PathBuf,
        params: HashSet<Parameter>,
    }

    impl Options {
        fn new() -> Result<Options, Error> {
            let args = env::args().skip(1);   
            Self::from_iter(args)         
        }

        fn from_iter<I: Iterator>(iter: I) -> Result<Options, Error> where I::Item: Into<String> {
            let mut params = HashSet::with_capacity(3);
            params.insert(Parameter::Chars);

            let mut file_name = String::new();

            for arg in iter {
                match arg.into().as_str() {
                    "-c" => { params.insert(Parameter::Chars); },
                    "-l" => { params.insert(Parameter::Lines); },
                    "-w" => { params.insert(Parameter::Words); },
                    value => {
                        file_name = value.to_string();
                        break;
                    }
                }
            }

            let file_name = PathBuf::from_str(&file_name)?;
            if file_name.extension().unwrap() != "txt" {
                return Err(Error::InvalidFileExtension);
            }

            return Ok(Options { file_name, params });
        }

        fn process(&self) -> Result<(), Error> {
            let mut file = std::fs::File::open(&self.file_name)?;
            let mut data = String::new();
            file.read_to_string(&mut data)?;

            let graphemes: Vec<&str>;

            if self.params.contains(&Parameter::Chars) || self.params.contains(&Parameter::Lines)  {
                graphemes = data.graphemes(true).collect::<Vec<&str>>();
            } else {
                graphemes = vec![];
            }

            let cores = thread::available_parallelism()?.get();
            let chunk_size = (data.len() + cores - 1) / cores;
            let chunks = graphemes.chunks(chunk_size);

            for param in &self.params {
                match param {
                    Parameter::Chars => {
                        let chars = Self::find_chars(chunks.clone());
                        println!("chars {chars}");
                    },
                    Parameter::Lines => {
                        let lines = Self::find_lines(chunks.clone());
                        println!("lines {lines}");
                    },
                    Parameter::Words => {
                        let words = data.unicode_words().count();
                        println!("words {words}");
                    }
                }
            }

            Ok(())
        }

        fn find_chars<'a>(chunks: Chunks<'_, &'a str>) -> u64 {
            Self::find_pattern(chunks, |ch|{ 
                !(ch == " " || ch == "\n" || ch == "\t")
            })
        }

        fn find_lines<'a>(chunks: Chunks<'_, &'a str>) -> u64 {
            let lines = Self::find_pattern(chunks, |ch|{ 
                ch == "\n"
            });
            lines + 1
        }

        fn find_pattern<'a, F: Fn(&str) -> bool + Send + Copy>(chunks: Chunks<'_, &'a str>, pattern: F) -> u64 {
            let counter = Arc::new(AtomicU64::new(0));
            thread::scope(|s| {
                for chunk in chunks {
                    let counter = Arc::clone(&counter);
                    s.spawn(move ||{
                        for ch in chunk {
                            if pattern(ch) {
                                counter.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    });
                }                
            });
            return counter.load(Ordering::Relaxed);
        }
    }

    pub fn process() -> Result<(), Error> {
        let options = Options::new()?;
        options.process()
    }
}
