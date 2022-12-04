#![feature(test)]
#![feature(iter_advance_by)]
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

struct MyIter<'a, T: Iterator<Item = &'a u8>> {
    iter: T
}

impl<'a, T: Iterator<Item = &'a u8>> Iterator for MyIter<'a, T> {
    type Item = [u16; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let mut result: [u16; 4] = [0; 4];
        for r in result.iter_mut() {
            let c1 = *self.iter.next()?;
            let c2 = *self.iter.next().unwrap();
            let double_digit = c2 >= b'0';
            self.iter.advance_by(double_digit as usize).unwrap();
            *r = u16::from_be_bytes([c1, c2]);
            if !double_digit {
                *r >>= 8;
            }
        }
        Some(result)
    }
}

fn solve_1(input: &[u8]) -> String {
    MyIter { iter: input.into_iter() }
        .filter(|[x1, x2, y1, y2]| {
            (x1 <= y1 && y2 <= x2) || (y1 <= x1 && x2 <= y2)
        })
        .count()
        .to_string()
}

fn solve_2(input: &[u8]) -> String {
    MyIter { iter: input.into_iter() }
        .filter(|[x1, x2, y1, y2]| {
            x1 <= y2 && x2 >= y1
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use super::*;

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
