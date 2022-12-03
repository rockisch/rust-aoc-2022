#![feature(test)]
#![feature(iter_array_chunks)]
#![feature(hash_drain_filter)]
use std::{collections::{HashMap, HashSet}, ops::Index};
extern crate test;

pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
pub const DATA: &[u8] = include_bytes!("data.txt");

fn main() {
    let r1 = solve_1(DATA);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &[u8]) -> String {
    let mut hash = HashSet::<u8>::new();
    input.split(|&v| v == b'\n')
        .filter(|v| !v.is_empty())
        .map(|v| {
            let (b1, b2) = v.split_at(v.len()/2);
            hash.clear();
            hash.extend(b1);
            let r = b2.iter().find(|v| hash.contains(v)).unwrap();
            if r.is_ascii_uppercase() {
                (r - b'A' + 27) as u16
            } else {
                (r - b'a' + 1) as u16
            }
        })
        .sum::<u16>()
        .to_string()
}

fn solve_2(input: &[u8]) -> String {
    let mut hash = HashSet::<u8>::new();
    let mut hash_aux = HashSet::<u8>::new();
    input.split(|&v| v == b'\n')
        .filter(|v| !v.is_empty())
        .array_chunks::<3>()
        .map(|v| {
            let [b1, b2, b3] = v;
            hash.clear();
            hash.extend(b1);
            hash_aux.clear();
            hash_aux.extend(b2);
            hash.drain_filter(|v| !hash_aux.contains(v));
            hash_aux.clear();
            hash_aux.extend(b3);
            hash.drain_filter(|v| !hash_aux.contains(v));
            let r = hash.iter().next().unwrap();
            if r.is_ascii_uppercase() {
                (r - b'A' + 27) as u16
            } else {
                (r - b'a' + 1) as u16
            }
        })
        .sum::<u16>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve_1(EXAMPLE), "157");
        assert_eq!(solve_1(DATA), "7674");
        assert_eq!(solve_2(EXAMPLE), "70");
        // assert_eq!(solve_2(DATA), "");
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
