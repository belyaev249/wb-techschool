fn main() {
    for i in (1..5).filter(|_|{true}).map_double() {
        print!("{i} ");
    }

    println!();

    for i in vec![String::from("a"),String::from("b"),String::from("c")].into_iter().filter(|_|{true}).map_double() {
        print!("{i} ");
    }
}

trait Double {
    fn double(&self) -> Self;
}

impl Double for String {
    fn double(&self) -> Self {
        format!("{}{}", self, self)
    }
}

impl Double for i32 {
    fn double(&self) -> Self {
        self << 1
    }
}

// В раст любой итератор - адаптер
// Он берет итерфейс предыдущего итератора и предоставляет новый

struct MapDoubleIterator<I, T> where T: Double, I: Iterator<Item = T> {
    iter: I
}

impl<I, T> Iterator for MapDoubleIterator<I, T> where T: Double, I: Iterator<Item = T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(value) => Some(value.double()),
            _ => None
        }
    }
}

trait MapDouble<T: Double>: Iterator<Item = T> + Sized {
    fn map_double(self) -> MapDoubleIterator<Self, T> {
        MapDoubleIterator { iter: self }
    }
}

impl<T: Double, I: Iterator<Item = T>> MapDouble<T> for I {}

// И схематичный пример
// Библиотека предоставляет интерфейс нового модуля, который использует интерфейс старого

// let n = example::NewService;
// n.new_method();

mod example_lib {
    pub struct NewService;

    impl OldInterface for NewService {}
    impl NewInterface for NewService {}

    pub trait NewInterface: OldInterface {
        fn new_method(&self) -> String {
             format!("{} + New value", self.old_method())
        }
    }

    trait OldInterface {
        fn old_method(&self) -> String {
            String::from("Old value")
        }
    }
}