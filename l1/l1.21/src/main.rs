use bigint::BigInt;
mod tests;

fn main() {
    let b1 = BigInt::from_str("123");
    let b2 = BigInt::from_str("123");
    let b3 = b1 + b2;
    println!("{:?}\n", b3.into_str());
    assert_eq!(b3.into_str(), "246");

    let b1 = BigInt::from_str("123");
    let b2 = BigInt::from_str("123");
    let b3 = b1 - b2;
    println!("{:?}\n", b3.into_str());
    assert_eq!(b3.into_str(), "0");

     // u128::max
    let b1 = BigInt::from_str("340282366920938463463374607431768211455");
    let b2 = BigInt::from_str("340282366920938463463374607431768211455");
    let b3 = b1 * b2;
    println!("{:?}\n", b3.into_str());
    // u256
    assert_eq!(b3.into_str(), "115792089237316195423570985008687907852589419931798687112530834793049593217025");

    let b1 = BigInt::from_str("121");
    let b2 = BigInt::from_str("11");
    let b3 = b1 / b2;
    println!("{:?}\n", b3.into_str());
    assert_eq!(b3.into_str(), "11");
}

pub mod bigint {
    use std::{cmp::*, ops::{Add, Div, Mul, Sub, Neg}};

    #[derive(Clone)]
    enum Sign {
        Positive, Negative
    }

    pub struct BigInt {
        sign: Sign,
        pub v: Vec<u8>
    }

    // Инициализация

    impl BigInt {
        pub fn from_str(str: &str) -> BigInt {
            let sign = &str[0..=0];
            let sign = match sign {
                "+" => Some(Sign::Positive),
                "-" => Some(Sign::Negative),
                _ => None
            };

            let v = match sign {
                Some(_) => &str[1..],
                _ => &str
            };

            let chars = v.chars();
            let mut v = Vec::<u8>::with_capacity(str.len());
            let mut is_prefix = true;

            for ch in chars {
                if ch == '0' && is_prefix {
                    continue;
                }
                if let Some(digit) = ch.to_digit(10) {
                    is_prefix = false;
                    v.push(digit as u8);
                }
            }

            let sign = sign.unwrap_or(Sign::Positive);

            if v.is_empty() {
                v = vec![0];
            }

            return BigInt { sign, v };
        }

        pub fn into_str(&self) -> String {
            let sign: &str = match self.sign {
                Sign::Negative => "-",
                _ => ""
            };
            format!("{}{}", sign, self.v.iter().map(|i| i.to_string().chars().collect::<Vec<_>>()).flatten().collect::<String>())
        }
    }

    // Арифметические операции

    impl Neg for BigInt {
        type Output = BigInt;
        fn neg(self) -> Self::Output {
            let sign = match &self.sign {
                Sign::Positive => Sign::Negative,
                Sign::Negative => Sign::Positive
            };
            BigInt { sign, v: self.v }
        }
    }

    impl Add for BigInt {
        type Output = BigInt;
        fn add(self, rhs: BigInt) -> Self::Output {
            let sign: Sign;
            let v: Vec<u8>;

            let mut lhs_iter = self.v.iter().rev();
            let mut rhs_iter = rhs.v.iter().rev();

            match (&self.sign, &rhs.sign) {
                (Sign::Negative, Sign::Positive) | (Sign::Positive, Sign::Negative) => {
                    match  cmp(&self.v, &rhs.v) {
                        Ordering::Equal => {
                            v = vec![0];
                            sign = Sign::Positive;
                        },
                        Ordering::Greater => {
                            v = slice_sub(&mut lhs_iter, &mut rhs_iter);
                            sign = self.sign.clone();
                        },
                        Ordering::Less => {
                            v = slice_sub(&mut rhs_iter, &mut lhs_iter);
                            sign = rhs.sign.clone();
                        }                        
                    }
                },
                _ => {
                    v = slice_add(&mut lhs_iter, &mut rhs_iter);
                    sign = self.sign.clone();
                }
            }

            let v = v.into_iter().rev().collect();
            return BigInt { sign, v };
        }
    }

    impl Sub for BigInt {
        type Output = BigInt;
        fn sub(self, rhs: BigInt) -> Self::Output {
            self.add(-rhs)
        }
    }

    impl Mul for BigInt {
        type Output = BigInt;
        fn mul(self, rhs: BigInt) -> Self::Output {
            let sign = match (self.sign, rhs.sign) {
                (Sign::Negative, Sign::Positive) | (Sign::Positive, Sign::Negative) => Sign::Negative,
                _ => Sign::Positive
            };
            let mut lhs_iter = self.v.iter().rev();
            let mut rhs_iter = rhs.v.iter().rev();
            let v = slice_mul(&mut lhs_iter, &mut rhs_iter).into_iter().rev().collect();
            return  BigInt { sign, v };
        }
    }

    impl Div for BigInt {
        type Output = BigInt;
        fn div(self, rhs: BigInt) -> Self::Output {
            if rhs.v.is_empty() || rhs.v == [0] {
                panic!("The division by zero operation is not defined");
            }
            let v:Vec<u8>; 
            let sign: Sign;
            if cmp(&self.v, &rhs.v) == Ordering::Less {
                v = vec![0];
                sign = Sign::Positive;
            } else {
                v = slice_div(&self.v, &rhs.v);
                sign = match (self.sign, rhs.sign) {
                    (Sign::Negative, Sign::Positive) | (Sign::Positive, Sign::Negative) => Sign::Negative,
                    _ => Sign::Positive
                };
            }
            return  BigInt { sign, v };
        }
    }

    // Операции сложения, вычитания и умножения сделаны столбиком
    // Деление - бинарный поиск. Каждый шаг умножаем потенциальное частное на делитель, сравниваем с исходным числом - делимым

    fn slice_add<'a, I>(lhs: &mut I, rhs: &mut I) -> Vec<u8> where I: DoubleEndedIterator<Item = &'a u8> + ExactSizeIterator {
        let mut v = Vec::with_capacity(lhs.len().max(rhs.len()) + 1);
        let mut add = 0u8;
        loop {
            let (lhs, rhs) = (lhs.next(), rhs.next());
            let mut value = match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => lhs + rhs + add,
                (Some(lhs), None) => lhs + add,
                (None, Some(rhs)) => rhs + add,
                _ => break
            };
            if value > 9 {
                add = value / 10;
                value = value % 10;
            } else {
                add = 0;
            }
            v.push(value);
        }
        if add != 0 {
            v.push(add);
        }
        return v;
    }

    fn slice_sub<'a, I>(lhs: &mut I, rhs: &mut I) -> Vec<u8> where I: DoubleEndedIterator<Item = &'a u8> + ExactSizeIterator {
        let mut v: Vec<u8> = Vec::with_capacity(lhs.len().max(rhs.len()));
        let mut sub = 0u8;
        loop {
            let (lhs, rhs) = match (lhs.next(), rhs.next()) {
                (Some(lhs), Some(rhs)) => (*lhs, rhs + sub),
                (Some(lhs), None) => (*lhs, sub),
                _ => { break; }
            };
            let value: u8;
            if lhs < rhs {
                sub = 1;
                value = lhs + (10 - rhs);
            } else {
                sub = 0;
                value = lhs - rhs;
            }
            v.push(value);
        }
        while v.last() == Some(&0) {
            v.pop();
        }
        return v;
    }

    fn slice_mul<'a, I>(lhs: &mut I, rhs: &mut I) -> Vec<u8> where I: DoubleEndedIterator<Item = &'a u8> + ExactSizeIterator + Clone {
        let mut v = Vec::with_capacity(lhs.len() * 2 + rhs.len() * 2);
        let tmp_capacity = lhs.len() + 1;
        let mut idx = -1;
        for rhs in rhs {
            let mut tmp = Vec::with_capacity(tmp_capacity);
            let mut add = 0u8;
            idx += 1;
            for _ in 0..=idx-1 {
                tmp.push(0);
            }
            for lhs in lhs.clone() {
                let mut value = rhs * lhs + add;
                add = value / 10;
                value = value % 10;
                tmp.push(value);
            }
            if add != 0 {
                tmp.push(add);
            }
            v = slice_add(&mut v.iter(), &mut tmp.iter());
        }
        return v;
    }

    fn slice_div(lhs: &Vec<u8>, rhs: &Vec<u8>) -> Vec<u8> {
        let mut left = vec![1];
        let mut right = lhs.clone();

        while cmp(&left, &right) == Ordering::Less {
            let left_right_sum = slice_add(&mut left.iter().rev(), &mut right.iter().rev());
            let left_right_sum: Vec<u8> = slice_add(&mut left_right_sum.iter(), &mut [1].iter());
            let middle = slice_div_by_2(&mut left_right_sum.iter().rev());

            let mul: Vec<u8> = slice_mul(&mut middle.iter().rev(), &mut rhs.iter().rev()).into_iter().rev().collect();
            match cmp(lhs, &mul) {
                Ordering::Equal => return middle,
                Ordering::Greater => { left = middle },
                Ordering::Less => {
                    let middle: Vec<u8> = slice_sub(&mut middle.iter().rev(), &mut [1].iter().rev());
                    let middle: Vec<u8> = middle.into_iter().rev().collect();
                    right = middle;
                }
            }
        }
        return left;
    }

    // Вспомогательные методы
    // Деление на 2 столбиком и сравнение векторов

    fn slice_div_by_2<'a, I>(lhs: &mut I) -> Vec<u8> where I: Iterator<Item = &'a u8> + ExactSizeIterator {
        let mut v = Vec::with_capacity(lhs.len());
        let mut div = 0u8;
        loop {
            let mut value = 0u8;
            let mut with_skip = false;
            if let Some(value1) = lhs.next() {
                let tmp = 10 * div + *value1;
                if tmp == 0 {
                    value = 0;
                } else if tmp > 1 {
                    value = tmp;
                } else if let Some(value2) = lhs.next() {
                    value = tmp * 10 + value2;
                    with_skip = true;
                }
            } else {
                break;
            }
            let quotient = value / 2;
            let quotient1 = quotient / 10;
            let quotient2 = quotient % 10;

            if quotient1 > 0 {
                v.append(&mut vec![quotient1, quotient2]);
            } else {
                if with_skip && v.len() != 0 {
                    v.push(0);
                }
                v.push(quotient2);
            }
            div = value - (quotient * 2);
        }
        return v;
    }

    fn cmp(lhs: &Vec<u8>, rhs: &Vec<u8>) -> Ordering {
        if lhs.len() != rhs.len() {
            return lhs.len().cmp(&rhs.len());
        }
        return lhs.cmp(&rhs);
    }
}