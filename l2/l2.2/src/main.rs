use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // "a4bc2d5e" => "aaaabccddddde"
    let str = "a4bc2d5e".graphemes(true);
    let iter = parse_string::ParseString::from_iter(str).unwrap();
    let str = String::from(iter);
    assert_eq!(str, "aaaabccddddde");

    // "abcd" => "abcd"
    let str = "abcd".graphemes(true);
    let iter = parse_string::ParseString::from_iter(str).unwrap();
    let str = String::from(iter);
    assert_eq!(str, "abcd");

    // "" => ""
    let str = "".graphemes(true);
    let iter = parse_string::ParseString::from_iter(str).unwrap();
    let str = String::from(iter);
    assert_eq!(str, "");

    // qwe\4\5 => qwe45
    let str = r"qwe\4\5".graphemes(true);
    let iter = parse_string::ParseString::from_iter(str).unwrap();
    let str = String::from(iter);
    assert_eq!(str, "qwe45");

    // qwe\45 => qwe44444
    let str = r"qwe\45".graphemes(true);
    let iter = parse_string::ParseString::from_iter(str).unwrap();
    let str = String::from(iter);
    assert_eq!(str, r"qwe44444");

    // qwe\\5 => qwe\\\\\
    let str = r"qwe\\5".graphemes(true);
    let iter = parse_string::ParseString::from_iter(str).unwrap();
    let str = String::from(iter);
    assert_eq!(str, r"qwe\\\\\");
}

mod parse_string {
    use std::{fmt::Debug, iter::repeat};

    pub enum Error {
        StartsWithNumber
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let msg = match self {
                Error::StartsWithNumber => "А packed string cannot starts with a number"
            };
            write!(f, "{msg}")?;
            Ok(())
        }
    }

    #[derive(PartialEq, Eq, Clone)]
    enum State {
        Init, Char(String), Num(usize), Escape
    }

    impl<'a> From<&'a str> for State {
        fn from(value: &'a str) -> State {
            if let Ok(num) = value.parse::<usize>() {
                return State::Num(num)
            }
            if value == r"\" { return State::Escape }
            return State::Char(value.to_string())
        }
    }

    pub struct ParseString<'a, I: Iterator<Item = &'a str>> {
        prev_char: String,
        state: State,
        iter: I,
    }

    impl<'a, I: Iterator<Item = &'a str> + Clone> ParseString<'a, I> {
        pub fn from_iter(value: I) -> Result<ParseString<'a, I>, Error> {
            if let Some(init_state) = value.clone().nth(0) {
                let init_state = State::from(init_state);
                if let State::Num(_) = init_state {
                    return Err(Error::StartsWithNumber);
                }
            }
            Ok(ParseString { state: State::Init, iter: value, prev_char: "".to_string() })
        }
    }

    impl<'a, I: Iterator<Item = &'a str>> From<ParseString<'a, I>> for String {
        fn from(value: ParseString<'a, I>) -> String {
            value.collect()
        }
    }

    impl<'a, I> Iterator for ParseString<'a, I> where I: Iterator<Item = &'a str> {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            while let Some(next) = self.iter.next() {
                let state = State::from(next);
                match (self.state.clone(), state) {
                    (State::Init, State::Num(_)) => { panic!(); },
                    (State::Init, state) => { 
                        self.state = state;
                    },
                    (State::Num(num1), State::Num(num2)) => {
                        self.state = State::Num(num1 * 10 + num2);
                    },
                    (State::Num(num), state) => {
                        let value = repeat(self.prev_char.clone()).take(num);
                        self.state = state;
                        return Some(value.collect());
                    },
                    (State::Char(ch), State::Num(num)) => {
                        self.prev_char = ch;
                        self.state = State::Num(num);
                    },
                    (State::Char(ch1), State::Char(ch2)) => {
                        self.state = State::Char(ch2);
                        return Some(ch1.to_string());
                    },
                    (State::Char(ch), State::Escape) => {
                        self.state = State::Escape;
                        return Some(format!(r"{ch}"));
                    },
                    (State::Escape, State::Num(num)) => {
                        self.state = State::Char(num.to_string());
                    },
                    (State::Escape, State::Char(ch)) => {
                        self.state = State::Char(ch);
                    },
                    (State::Escape, State::Escape) => {
                        self.state = State::Char(r"\".to_string());
                    }
                    _ => {}
                }
            }
            
            match self.state.clone() {
                State::Char(ch) => {
                    self.state = State::Init;
                    return Some(ch.to_string());
                },
                State::Num(num) => {
                    self.state = State::Init;
                    return Some(repeat(self.prev_char.clone()).take(num).collect());
                },
                _ => {}
            }
            return None;
        } 
    }

    #[cfg(test)]
    mod tests {
        use unicode_segmentation::UnicodeSegmentation;

        fn test(lhs: &str, rhs: &str) {
            let lhs = lhs.graphemes(true);
            let lhs = super::ParseString::from_iter(lhs).unwrap();
            let lhs = String::from(lhs);
            assert_eq!(lhs, rhs);
        }

        #[test]
        fn starts_with_num_test() {
            let lhs = "123".graphemes(true);
            let lhs = super::ParseString::from_iter(lhs);
            assert!(lhs.is_err());
        }

        #[test]
        fn test1() {
            test("a1b2c3d4", "abbcccdddd");
            test("ab2c3d4", "abbcccdddd");
            test("b3b7", "bbbbbbbbbb");
            test("bb3b3b3", "bbbbbbbbbb");
            test("a0b0c0a", "a");
        }

        #[test]
        fn test2() {
            test(r"a\a\", "aa");
            test(r"a\a\1", "aa1");
            test(r"a\a\\4b\3b3", r"aa\\\\b3bbb");
            test(r"a\a\\\4\b\3b3", r"aa\4b3bbb");
            test(r"a0b0c0aa\0", r"aa0");
        }
    }
}