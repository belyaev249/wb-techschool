fn main() {
    let str = "first second third";
    let rev_words_str: String = str.split_with_save_whitespace().rev().collect();
    println!("{}", &rev_words_str);
    assert_eq!(&rev_words_str, "third second first");

    let str = "snow dog sun";
    let rev_words_str: String = str.split_with_save_whitespace().rev().collect();
    println!("{}", &rev_words_str);
    assert_eq!(&rev_words_str, "sun dog snow");
}

struct SplitWithSaveWhitespaceIterator<'a> {
    is_whitespace: bool,
    inner: std::str::SplitWhitespace<'a>,
    idx: usize,
    len: usize,
}

impl<'a> Iterator for SplitWithSaveWhitespaceIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len <= self.idx {
            return None;
        }
        if !self.is_whitespace {
            let next = self.inner.next();
            self.idx += 1;
            self.is_whitespace = true;
            return next;
        } else {
            self.is_whitespace = false;
            return Some(" ");
        }
    }
}

impl<'a> DoubleEndedIterator for SplitWithSaveWhitespaceIterator<'a>  {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        if !self.is_whitespace {
            let next = self.inner.next_back();
            self.len -= 1;
            self.is_whitespace = true;
            return next;
        } else {
            self.is_whitespace = false;
            return Some(" ");
        }
    }
}

trait SplitWithSaveWhitespace {
    fn split_with_save_whitespace(&self) -> SplitWithSaveWhitespaceIterator<'_>;
}

impl SplitWithSaveWhitespace for &str {
    fn split_with_save_whitespace(&self) -> SplitWithSaveWhitespaceIterator<'_> {
        let len: usize = self.split_whitespace().count();
        SplitWithSaveWhitespaceIterator { is_whitespace: false, inner: self.split_whitespace(), idx: 0, len }
    }
}