use std::cmp::{Ord, Ordering};

fn main() {
    let x = [0,1,2];
    let idx = x.bin_search(0, x.len(), 1);
    println!("{:?}", idx);
    assert_eq!(idx, Ok(1));

    let x = [1,2,3,4,5,6];
    let idx = x.bin_search(0, x.len(), 3);
    println!("{:?}", idx);
    assert_eq!(idx, Ok(2));

    let x = [1,2,3];
    let idx = x.bin_search(0, x.len(), 0);
    println!("{:?}", idx);
    assert_eq!(idx, Err(0));
}

trait BinSearch<T> {
    // l - левая граница
    // r - правая граница
    // target - искомый элемент
    fn bin_search(&self, l: usize, r: usize, target: T) -> Result<usize, usize>;
}

impl<T: Ord> BinSearch<T> for [T] {
    // При успешном поиске элемента возвращается Ok({индекс элемента})
    // При отстутсвии элемента возвращается Err({индекс, который бы занимал искомый элемент в отсортированном массиве})
    fn bin_search(&self, l: usize, r: usize, target: T) -> Result<usize, usize> {
        if l >= r {
            if self[l] == target { 
                return Ok(l);
            } else {
                return Err(l);
            }
        }
        // Чтобы избежать переполнений
        let m = l + (r -l) / 2;

        match self[m].cmp(&target) {
            Ordering::Equal => { return Ok(m); },
            Ordering::Less => { return self.bin_search(m+1, r, target); },
            Ordering::Greater => { return self.bin_search(l, m, target); }
        }
    }
}
