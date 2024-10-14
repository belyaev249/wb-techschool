fn main() {
    grep::process().unwrap();
}

// Пример:
// cargo run -- абв -A=3 -i file.txt

mod grep {
    use regex::Regex;
    use clap::{Parser, arg};
    use std::fmt::Debug;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Read;

    pub enum Error {
        Unknown(String)
    }

    impl<T> From<T> for Error where T: ToString {
        fn from(value: T) -> Error {
            Error::Unknown(value.to_string())
        }
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            let msg = match self {
                Self::Unknown(msg) => msg
            };
            write!(f, "{msg}")?;
            Ok(())
        }
    }

    // -A — "after" печатать +N строк после совпадения
    // -B — "before" печатать +N строк до совпадения
    // -C — "context" (A+B) печатать ±N строк вокруг совпадения
    // -c — "count" (количество строк)
    // -i — "ignore_case" (игнорировать регистр)
    // -v — "invert" (вместо совпадения, исключать)
    // -F — "fixed", точное совпадение со строкой, не паттерн
    // -n — "line_number", напечатать номер строки

    #[derive(Parser, Debug, Clone)]
    #[command(disable_help_flag = true)]
    struct Args {
        re: String,
        #[arg(default_value_t=0, short='A', long, help="")]
        after: usize,
        #[arg(default_value_t=0, short='B', long, help="")]
        before: usize,
        #[arg(short='C', long, help="")]
        context: Option<usize>,
        #[arg(short='c', long, help="")]
        count: bool,
        #[arg(short='i', long, help="")]
        ignore_case: bool,
        #[arg(short='v', long, help="")]
        invert: bool,
        #[arg(short='F', long, help="")]
        fixed: bool,
        #[arg(short='n', long, help="")]
        line_number: bool,
        input_file_path: PathBuf
    }

    pub fn process() -> Result<(), Error> {
        let args = Args::parse();

        let mut input_file = File::open(args.input_file_path)?;
        let mut input_buffer = String::new();
        input_file.read_to_string(&mut input_buffer)?;

        let re = Regex::new(&args.re)?;
        let mut count = 0;

        let lines = input_buffer.lines();
        let lines_vec = lines.to_owned().collect::<Vec<_>>();

        for (idx, line) in lines.enumerate() {
            let mut line = line.to_owned();
            if args.ignore_case {
                line = line.to_lowercase();
            }
            let is_matches = is_str_matches(&line, args.fixed, args.invert, &re, &args.re);
            if is_matches {
                if args.count {
                    count += 1;
                    continue;
                }
                if args.line_number {
                    println!("{}", idx + 1);
                    continue;
                }
                let before = args.context.unwrap_or(args.before);
                let after = args.context.unwrap_or(args.after);
                for i in (idx as i32)-(before as i32)..=(idx as i32)+(after as i32) {
                    // let peek_line = lines.nth(i);
                    if i < 0 { continue; }
                    let l = lines_vec.get(i as usize);
                    if let Some(l) = l {
                        println!("{l}");
                    }
                }
                println!();
            }
        }
        if args.count {
            println!("{count}");
        }

        Ok(())
    }

    fn is_str_matches(str: &str, is_fixed: bool, is_invert: bool, re: &Regex, e: &str) -> bool {
        let is_matches = if is_fixed {
            str.contains(e)
        } else {
            re.is_match(str)
        };
        return is_matches ^ is_invert;
    }
}