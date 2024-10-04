use std::{collections::HashSet, io::stdin};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    loop {
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap();
        let input_str = input_str.trim();
        let is_unique_by_chars = is_unique_by_chars(&input_str);
        let is_unique_by_graphemes = is_unique_by_graphemes(&input_str);
        println!("By chars: {is_unique_by_chars}, by graphemes: {is_unique_by_graphemes}\n");
    }
}

// Если проверять уникальность по символам - может отработать неправильно
// Например, строка "y̆y" будет отмечена как повторяющаяся

fn is_unique_by_chars(str: &str) -> bool {
    let mut hs = HashSet::with_capacity(str.len());
    for ch in str.chars() {
        let ch = ch.to_string();
        if !(hs.insert(ch.to_lowercase()) | hs.insert(ch.to_uppercase())) {
            return false;
        }
    }
    return true;
}

// С помощью стороннего крейта unicode_segmentation

fn is_unique_by_graphemes(str: &str) -> bool {
    let mut hs = HashSet::with_capacity(str.len());
    for g in str.graphemes(true) {
        if !(hs.insert(g.to_lowercase()) | hs.insert(g.to_uppercase())) {
            return false;
        }
    }
    return true;
}