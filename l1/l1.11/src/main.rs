use std::fmt::Debug;
use std::ops::Rem;
use std::collections::HashMap;

fn main() {
    let x = [-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let x = flat_by_bounds(&x);
    
    for (key, value) in x {
        println!("[{}0, {}0): {:?}", key, key + 1, value);
    }
}

fn flat_by_bounds(x: &[f64]) -> HashMap::<i128, Vec<f64>> {
    let mut map = HashMap::<i128, Vec<f64>>::with_capacity(x.len());
    for value in x {
        let lower_bound = get_lower_bound(value);
        if let Some(map_value) = map.get_mut(&lower_bound) {
            map_value.push(*value);
        } else {
            map.insert(lower_bound, vec![*value]);
        }
    }
    return map;
}

fn get_lower_bound(x: &f64) -> i128 {
    let mut y = *x as i128 / 10;
    if x.rem(10.0) < 0.0 {
        y -= 1;
    }
    // Для поддержки больших чисел границы имеют сокращенный вид
    // Например, [10; 20) = [1; 2)
    // Последний ноль будет дописан при выводе
    return y;
}