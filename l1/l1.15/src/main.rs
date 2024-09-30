use std::cmp::Ord;

fn main() {
    let x = &mut [2,1];
    x.quick_sort();
    assert_eq!(x, &[1,2]);

    let x = &mut [1111,1,44,3];
    x.quick_sort();
    assert_eq!(x, &[1,3,44,1111]);

    let mut x = vec![1,2,3,4,1,2,3,4,4,3,2,1];
    x.quick_sort();
    assert_eq!(x, &[1,1,1,2,2,2,3,3,3,4,4,4])
}

trait QuickSort {
    fn quick_sort_bounds(&mut self, left: usize, right: usize);
    fn quick_sort(&mut self);
}

impl<T: Ord + Clone> QuickSort for [T] {
    fn quick_sort(&mut self) {
        let len = self.len();
        if len <= 1 {
            return;
        }
        self.quick_sort_bounds(0, len-1);
    }

    fn quick_sort_bounds(&mut self, left: usize, right: usize) {
        if left >= right {
            return;
        }
    
        let i = (left + right + 1) / 2;
        let p = self[i].clone();
    
        let mut l = left;
        let mut r = right;
    
        while l <= r {
            while self[l] < p {
                l += 1;
            }
            while self[r] > p {
                r -= 1;
            }
            if l <= r {
                self.swap(l, r);
                l += 1;
                r -= 1;
            }
        }
    
        self.quick_sort_bounds(left, r);
        self.quick_sort_bounds(l, right);
    }
}