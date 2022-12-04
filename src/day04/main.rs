#![feature(test)]
#![feature(iter_advance_by)]
#![feature(exact_size_is_empty)]
extern crate test;

pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
pub const DATA: &[u8] = include_bytes!("data.txt");

fn main() {
    println!("Day 4");
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

struct ItemsIter<'a, T: ExactSizeIterator<Item = &'a u8>> {
    iter: T,
}

impl<'a, T: ExactSizeIterator<Item = &'a u8>> Iterator for ItemsIter<'a, T> {
    type Item = [u16; 4];

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_empty() {
            return None;
        }
        let mut result: [u16; 4] = [0; 4];
        for r in result.iter_mut() {
            let c1 = unsafe { *self.iter.next().unwrap_unchecked() };
            let c2 = unsafe { *self.iter.next().unwrap_unchecked() };
            *r = u16::from_be_bytes([c1, c2]);
            let double_digit = c2 & 0b10000 != 0;
            if double_digit {
                unsafe { self.iter.next().unwrap_unchecked() };
            } else {
                *r >>= 8;
            }
        }
        Some(result)
    }
}

fn solve_1(input: &[u8]) -> String {
    ItemsIter {
        iter: input.into_iter(),
    }
    .filter(|[x1, x2, y1, y2]| (x1 <= y1 && y2 <= x2) || (y1 <= x1 && x2 <= y2))
    .count()
    .to_string()
}

fn solve_2(input: &[u8]) -> String {
    ItemsIter {
        iter: input.into_iter(),
    }
    .filter(|[x1, x2, y1, y2]| x1 <= y2 && x2 >= y1)
    .count()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn test_4() {
        assert_eq!(solve_1(EXAMPLE), "2");
        assert_eq!(solve_1(DATA), "485");
        assert_eq!(solve_2(EXAMPLE), "4");
        assert_eq!(solve_2(DATA), "857");
    }

    #[bench]
    fn bench_4_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_4_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
