use std::collections::HashMap;

fn main() {
    let words = ["пятак", "пятка", "тяпка", "листок", "слиток", "стОлик"];
    let anagrams = anagrams(&words);
    println!("{anagrams:?}");
}

fn anagrams<'a>(words: &'a [&str]) -> HashMap::<String, Vec<String>> {
    let mut keys_dictionary = HashMap::<String, (String, usize)>::with_capacity(words.len());
    for word in words {
        let word_lowercased = word.to_lowercase();
        let key = word_lowercased.as_str().sorted();
        if !keys_dictionary.contains_key(&key) {
            keys_dictionary.insert(key, (word_lowercased, 1));
        } else {
            keys_dictionary.get_mut(&key).unwrap().1 += 1;
        }
    }
    let mut anagrams_dictionary = HashMap::<String, Vec<String>>::with_capacity(words.len());
    for word in words {
        let word_lowercased = word.to_lowercase();
        let word_sorted = word_lowercased.as_str().sorted();
        let key = &keys_dictionary[&word_sorted];
        if key.1 == 1 {
            continue;
        }
        if !anagrams_dictionary.contains_key(&key.0) {
            anagrams_dictionary.insert(key.0.clone(),  Vec::new());
        }
        let value = anagrams_dictionary.get_mut(&key.0).unwrap();
        value.sort_push(word_lowercased);
    }
    return anagrams_dictionary;
}

trait Sort {
    fn sorted(&self) -> String;
}

impl Sort for &str {
    fn sorted(&self) -> String {
        let mut sorted_chars = self.chars().collect::<Vec<char>>();
        sorted_chars.sort();
        return sorted_chars.iter().collect::<String>();
    }
}

trait SortPush {
    type Item;
    fn sort_push(&mut self, value: Self::Item);
}

impl<T: std::cmp::Ord> SortPush for Vec<T> {
    type Item = T;
    fn sort_push(&mut self, value: Self::Item) {
        if self.len() == 0 {
            self.push(value);
            return;
        }
        if &value <= &self[0] {
            self.insert(0, value);
            return;
        }
        if &value > &self[self.len()-1] {
            self.push(value);
            return;
        }
        let mut left: usize = 0;
        let mut right: usize = self.len() - 1;

        while left < right {
            let _idx = (left + right + 1)/2;
            let _value = &self[_idx];
            match &value.cmp(_value) {
                std::cmp::Ordering::Equal => {
                    left = _idx;
                    break;
                },
                std::cmp::Ordering::Less => {
                    right = _idx -1;
                }
                std::cmp::Ordering::Greater => {
                    left = _idx ;
                }
            }
        }
        self.insert(left + 1, value);
    }
}