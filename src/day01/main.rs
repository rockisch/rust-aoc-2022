#![feature(test)]
#![feature(split_as_slice)]
extern crate test;

use std::cmp;
use aoc_rust::utils;

pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
pub const DATA: &[u8] = include_bytes!("data.txt");

fn main() {
    let r1 = solve_1(DATA);
    let r2 = solve_2(DATA);
    println!("RESULTS: {r1} {r2}");
}

fn solve_1(input: &[u8]) -> String {
    let mut max = 0;
    let mut acc = 0;
    for v in input.split(|&v| v == b'\n') {
        if v.is_empty() {
            max = cmp::max(max, acc);
            acc = 0;
        } else {
            acc += utils::parse_bytes_fast(v);
        }
    }
    max.to_string()
}

fn solve_2(input: &[u8]) -> String {
    let mut arr: [u32; 4] = [0; 4];
    for v in input.split(|&v| v == b'\n') {
        if v.is_empty() {
            arr.sort_unstable();
            arr[0] = 0;
        } else {
            arr[0] += utils::parse_bytes_fast(v);
        }
    }
    arr[1..].iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve_1(EXAMPLE), "24000");
        assert_eq!(solve_1(DATA), "70116");
        assert_eq!(solve_2(EXAMPLE), "45000");
        assert_eq!(solve_2(DATA), "206582");
    }

    #[bench]
    fn bench_solve_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_solve_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }
}
