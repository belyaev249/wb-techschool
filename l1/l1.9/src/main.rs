fn main() {
    // 101
    let num = 0b101;

    // 100
    let xor_num = xor_ith(num, 0);
    
    assert_eq!(0b100, xor_num);
}

// Считается, что порядок битов справа налево. Например:
// 1 0 1 0 0 0 - биты
// 5 4 3 2 1 0 - индексы

fn xor_ith(num: i64, i: usize) -> i64 {
    let xor: i64 = 1 << i;
    
    // Индекс больше, чем длина числа взять нельзя
    if xor > num { panic!(); }

    num ^ xor
}