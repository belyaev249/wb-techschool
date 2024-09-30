use std::{collections::HashSet, hash::Hash};

fn main() {
    // Если в задании можно пользоваться библиотечным intersection:
    let a: HashSet<i32> = HashSet::<i32>::from([1,2,3,5]);
    let b: HashSet<i32> = HashSet::<i32>::from([2,5,8,9]);
    let x: HashSet<_> = a.intersection(&b).collect();
    assert_eq!([2,5].iter().collect::<HashSet<_>>(), x);

    // Если на векторах:
    let a = [1,2,3,5];
    let b = [8,9,5,2];
    let x: HashSet<_> = a.intersection(&b).collect();
    println!("{:?}", x);
    assert_eq!([2,5].iter().collect::<HashSet<_>>(), x);
}

struct Intersection<'a, T: Sized> {
    iter: std::slice::Iter<'a, T>,
    other: std::slice::Iter<'a, T>,
    map: HashSet<&'a T>
}

impl<'a, T: Sized + Eq + Hash> Iterator for Intersection<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(x) = self.iter.next() {
            self.map.insert(x);
        }

        while let Some(x) = self.other.next() {
            if let Some(x) = self.map.get(x) {
                return Some(x);
            }
        }

        return None;
    }
}

trait Intersect<T> {
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<T>;
}

impl<T> Intersect<T> for [T] {
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<T> {
        Intersection { iter: self.iter(), other: other.iter(), map: HashSet::new() }
    }
}