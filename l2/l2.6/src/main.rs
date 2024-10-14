fn main() {
    cut::process();
}

// Если не указан путь к файлу, строки считаются из стандартного ввода

// Примеры:
// cargo run -- [options]     | input                  | output
// cargo run -- -f 1-2 -d "," | 1,John Doe,Sales,50000 | 1,John Doe
// cargo run -- -f - -d " "   | 1 John Doe Sales 50000 | 1 John Doe Sales 50000

mod cut {
    use clap::{arg, Parser};
    use std::ops::RangeInclusive;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::{stdin, Read};
    use std::cmp::{min, max};

    #[derive(Parser)]
    struct Args {
        #[arg(default_value="-", short='f', long, help="")]
        fields: String,
        #[arg(default_value="\t", short='d', long, help="")]
        delimiter: String,
        #[arg(short='s', long, help="")]
        separated: bool,
        input_file_path: Option<PathBuf>,
    }

    pub fn process() {
        let args = Args::parse();
        let ranges = number_ranges(&args.fields);

        if let Some(file_path) = args.input_file_path {
            read_from_file(&file_path, &ranges, &args.delimiter, args.separated);
        } else {
            read_from_stdin(&ranges, &args.delimiter, args.separated);
        }
    }

    fn read_from_file(file_path: &PathBuf, ranges: &Vec<RangeInclusive<usize>>, delimiter: &str, separated: bool) {
        let mut file = File::open(file_path).unwrap();
        let mut input_str = String::new();
        file.read_to_string(&mut input_str).unwrap();
        read_str(&input_str, &ranges, delimiter, separated);
    }

    fn read_from_stdin(ranges: &Vec<RangeInclusive<usize>>, delimiter: &str, separated: bool) {
        println!("Начните вводить произвольные строки");
        println!("Выбранный разделитель для слов в строках: {delimiter}");
        loop {
            let mut input_str = String::new();
            stdin().read_line(&mut input_str).unwrap();
            read_str(&input_str, &ranges, delimiter, separated);
        }
    }

    fn read_str(s: &str, ranges: &Vec<RangeInclusive<usize>>, delimiter: &str, separated: bool) {
        let words = s.split(delimiter).collect::<Vec<&str>>();
        if separated && words.len() < 2 {
            return;
        }
        for range in ranges {
            let start = max(0, *range.start());
            let end = min(*range.end(), words.len()-1);
            let end = max(start, end);
            let r = &words[start..=end].join(delimiter);
            println!("{r}");
        }
    }

    // cut в качестве позиций колонок может принимать числа и диапазоны, разделенные запятыми
    // Например: 1,3-5,9-,-16 это диапазоны (1..=1), (3..=5), (9..), (..=16) 
    fn number_ranges(str: &str) -> Vec<RangeInclusive<usize>> {
        let mut ranges = vec![];
        for r in str.split(',') {
            let mut chars = r.chars();
            let mut num1 = vec![];
            let mut num2 = vec![];
            let mut has_range = false;
            while let Some(ch) = chars.next() {
                if ch.is_numeric() {
                    num1.push(ch.to_string());
                    continue;
                } else if ch == '-' {
                    has_range = true;
                }
                break;
            }
            while let Some(ch) = chars.next() {
                if ch.is_numeric() {
                    num2.push(ch.to_string());
                    continue;
                }
                break;
            }
            let num1 = num1.concat().parse::<usize>();
            let num2 = num2.concat().parse::<usize>();
            let range = match (num1, num2, has_range) {
                (Ok(num1), _, false) => num1-1..=num1-1,
                (Ok(num1), Ok(num2), true) => num1-1..=num2-1,
                (_, Ok(num2), true) => 0..=num2-1,
                (Ok(num1), _, true) => num1-1..=usize::MAX,
                _ => 0..=usize::MAX
            };
            ranges.push(range);
        }
        ranges.sort_by(|a, b| a.start().cmp(b.start()));
        let mut merged_ranges = vec![];
        if ranges.is_empty() {
            return vec![0..=usize::MAX];
        }
        let mut _range = ranges[0].clone();
        for range in &ranges {
            if _range.end() >= range.start() {
                let start = *_range.start();
                let end = *max(_range.end(), range.end());
                _range = start..=end;
            } else {
                merged_ranges.push(_range);
                _range = range.clone();
            }
        }
        merged_ranges.push(_range);
        return merged_ranges;
    }
}