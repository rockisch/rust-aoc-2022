// use std::io::{self, BufRead};

use std::{array::IntoIter};


pub struct FixedBinaryHeap<T, const N: usize> {
    arr: [T; N],
}

impl<T: Copy + Default + PartialOrd, const N: usize> FixedBinaryHeap<T, N> {
    pub fn new() -> Self {
        Self {
            arr: [T::default(); N],
        }
    }

    pub fn push(&mut self, mut v: T) {
        let mut i = 0;
        while i < N && v > unsafe { *self.arr.get_unchecked(i) } {
            i += 1;
        }
        while i > 0 {
            i -= 1;
            let j = unsafe { *self.arr.get_unchecked(i) };
            let j_ref = unsafe { self.arr.get_unchecked_mut(i) };
            (v, *j_ref) = (j, v);
        }
    }
}

impl<T, const N: usize> IntoIterator for FixedBinaryHeap<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}
