use std::iter::Enumerate;

fn main() {
    // Удаление с сохранением порядка
    let mut v = vec![1,2,4];
    v.remove(0);
    println!("{v:?}");

    // Удаление без сохранения порядка
    let mut v = vec![1,2,4];
    v.swap_remove(0);
    println!("{v:?}");

    // Удаление с сохранением порядка, в итераторе
    let v = vec![1,2,4];
    let iter = v.iter().enumerate().remove_at(0);
    for i in iter {
        println!("{i:?}");
    }
}

struct RemoveIndex<I: Iterator> {
    index: usize,
    iter: Enumerate<I>,
}

impl<I: Iterator> Iterator for RemoveIndex<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, value)) = self.iter.next() {
            if index != self.index {
                return Some(value);
            }
        }
        return None;
    }
}

trait RemoveIndexIterator<T, I: Iterator<Item = T>>: Iterator + Sized {
    fn remove_at(self, index: usize) -> RemoveIndex<I>;
}

impl<T, I: Iterator<Item = T>> RemoveIndexIterator<T, I> for Enumerate<I> {
    fn remove_at(self, index: usize) -> RemoveIndex<I> {
        RemoveIndex::<I> { index, iter: self }
    }
}