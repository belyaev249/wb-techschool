fn main() {
    sort::process().unwrap();
}

mod sort {
    use std::{cmp::Ordering, fmt::Debug};
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::{Read, Write};
    use clap::{arg, Parser};

    pub enum Error {
        Unknown(String)
    }

    impl From<std::io::Error> for Error {
        fn from(value: std::io::Error) -> Error {
            Error::Unknown(value.kind().to_string())
        }
    }

    impl From<std::convert::Infallible> for Error {
        fn from(value: std::convert::Infallible) -> Error {
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

    #[derive(Parser, Debug, Clone)]
    #[command(disable_help_flag = true)]
    struct Args {
        #[arg(short='k', long, help="")]
        key: Option<usize>,
        #[arg(short='n', long, help="")]
        numeric_sort: bool,
        #[arg(short='r', long, help="")]
        reverse: bool,
        #[arg(short='u', long, help="")]
        unique: bool,
        #[arg(short='M', long, help="")]
        month_sort: bool,
        #[arg(short='b', long, help="")]
        ignore_trailing_blanks: bool,
        #[arg(short='c', long, help="")]
        check: bool,
        #[arg(short='h', long, help="")]
        human_numeric_sort: bool,
        #[arg(short='o', long, help="")]
        output_file_path: PathBuf,
        input_file_path: PathBuf
    }

    impl Args {
        fn new() -> Args {
            Args::parse()
        }
    }

    pub fn process() -> Result<(), Error> {
        let args = Args::new();
        let mut file = File::open(&args.input_file_path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let mut buffer = buffer.lines().collect::<Vec<_>>();
        let input_buffer = buffer.clone();

        buffer.sort_by(|a, b|comparator(a, b, args.clone()));
        
        if args.check {
            return match buffer == input_buffer {
                true => Ok(()),
                _ => Err(Error::Unknown("Еhe input file is not sorted".to_string())),
            }
        }

        let mut file = File::create(&args.output_file_path)?;
        if args.unique {
            let mut mem = std::collections::HashSet::<&str>::new();
            for line in buffer {
                if !mem.contains(line) {
                    writeln!(&mut file, "{line}").unwrap();
                    mem.insert(line);
                }
            }            
        } else {
            for line in buffer {
                writeln!(&mut file, "{line}").unwrap();
            }
        }

        return Ok(())
    }

    fn comparator(a: &str, b: &str, args: Args) -> Ordering {
        let mut ord = Ordering::Equal;
        let mut a = a;
        let mut b = b;

        if args.ignore_trailing_blanks {
            a = a.trim_end();
            b = b.trim_end();
        }
        if let Some(k) = args.key {
            ord = ord.then(sort_by_key(a, b, k));
        }
        if args.numeric_sort {
            ord = ord.then(sort_by_numeric(a, b));
        }
        if args.month_sort {
            ord = ord.then(sort_by_month(a, b));
        }
        if args.reverse {
            ord = ord.reverse();
        }

        return ord; 
    }

    fn sort_by_key(a: &str, b: &str, k: usize) -> Ordering {
        let a = a.split_whitespace().nth(k);
        let b = b.split_whitespace().nth(k);
        a.cmp(&b)
    }

    fn sort_by_numeric(a: &str, b: &str) -> Ordering {
        fn parse_leading_num(s: &str) -> i32 {
            let s = s.chars().skip_while(|x| x == &' ').into_iter();
            s.map_while(|x| { (x.is_numeric() || x == '-').then(||x.to_string()) }).collect::<String>().parse::<i32>().unwrap_or(0)
        }
        let a = parse_leading_num(a);
        let b = parse_leading_num(b);
        return a.cmp(&b);
    }

    enum Month {
        Jan = 1, Feb = 2, Mar = 3, Apr = 4, May = 5, Jun = 6, Jul = 7, Aug = 8, Sep = 9, Oct = 10, Nov = 11, Dec = 12,
    }

    impl TryFrom<&str> for Month {
        type Error = Error;
        fn try_from(value: &str) -> Result<Month, Error> {
            let month = value.get(0..=2).unwrap_or("").to_lowercase();
            match month.as_str() {
                "jan" => Ok(Month::Jan), "feb" => Ok(Month::Feb), "mar" => Ok(Month::Mar),
                "apr" => Ok(Month::Apr), "may" => Ok(Month::May), "jun" => Ok(Month::Jun),
                "jul" => Ok(Month::Jul), "aug" => Ok(Month::Aug), "sep" => Ok(Month::Sep),
                "oct" => Ok(Month::Oct), "nov" => Ok(Month::Nov), "dec" => Ok(Month::Dec),
                _ => Err(Error::Unknown("".to_string()))
            }
        }
    }

    fn sort_by_month(a: &str, b: &str) -> Ordering {
        fn parse_leading_month(s: &str) -> Month {
            let s = s.chars().skip_while(|x| x == &' ').into_iter().take(3).collect::<String>();
            s.as_str().try_into().unwrap_or(Month::Jan)
        }
        let a = parse_leading_month(a) as u8;
        let b = parse_leading_month(b) as u8;
        return a.cmp(&b);
    }
}