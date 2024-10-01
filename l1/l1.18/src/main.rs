use std::io::stdin;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let input_str = "y̆esy̆es";
    let reverse_str = reverse_by_graphemes(&input_str);
    assert_eq!(reverse_str, "sey̆sey̆");

    let input_str = "a̐éö̲=-y̆♣Ж🍎";
    let reverse_str = reverse_by_graphemes(&input_str);
    assert_eq!(reverse_str, "🍎Ж♣y̆-=ö̲éa̐");

    println!("Начните вводить произвольные строки:");
    loop {
        let mut input_str = String::new();
        stdin().read_line(&mut input_str).unwrap();

        let reverse_str = reverse_by_graphemes(&input_str);
        println!("{}", reverse_str.trim());
        let reverse_str = reverse_by_chars(&input_str);
        println!("{}", reverse_str.trim());
        println!();
    }
}

// Переворачиваем строку по графемам
// С помощью внешнего крейта: unicode_segmentation
fn reverse_by_graphemes(input_str: &str) -> String {
    let reverse_str = input_str.graphemes(true).rev();
    String::from_iter(reverse_str)
}

// Переворачиваем строку по unicode символам
// Не для всех строк ожидаемый вывод
// y̆es - sĕy
// a̐é - 'e̐a
fn reverse_by_chars(input_str: &str) -> String {
    let reverse_str = input_str.chars().rev();
    String::from_iter(reverse_str)
}