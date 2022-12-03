#![feature(test)]
#![feature(iter_array_chunks)]
extern crate test;

pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
pub const DATA: &[u8] = include_bytes!("data.txt");

fn main() {
    let r1 = solve_1(EXAMPLE);
    println!("Part 1: {r1}");
    let r2 = solve_2(DATA);
    println!("Part 2: {r2}");
}

fn solve_1(input: &[u8]) -> String {
    input.split(|&v| v == b'\n')
        .filter(|v| !v.is_empty())
        .map(|v| {
            let (b1, b2) = v.split_at(v.len()/2);
            let (mut bit1, mut bit2): (u64, u64) = (0, 0);
            for (v1, v2) in b1.into_iter().zip(b2.into_iter()) {
                bit1 |= 1 << (v1 - 64);
                bit2 |= 1 << (v2 - 64);
            }
            let r = ((bit1 & bit2 ).trailing_zeros() + 64) as u16;
            if r >= 97 { r - (b'a' + 1) as u16 } else { r - (b'A' + 27) as u16 }
        })
        .sum::<u16>()
        .to_string()
}

fn solve_2(input: &[u8]) -> String {
    input.split(|&v| v == b'\n')
        .filter(|v| !v.is_empty())
        .array_chunks::<3>()
        .map(|[b1, b2, b3]| {
            let bit1: u64 = b1.into_iter().fold(0, |acc, &v| acc | (1 << (v - 64)));
            let bit2: u64 = b2.into_iter().fold(0, |acc, &v| acc | (1 << (v - 64)));
            let bit3: u64 = b3.into_iter().fold(0, |acc, &v| acc | (1 << (v - 64)));
            let r = ((bit1 & bit2 & bit3).trailing_zeros() + 64) as u16;
            if r >= 97 { r - (b'a' + 1) as u16 } else { r - (b'A' + 27) as u16 }
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
        assert_eq!(solve_2(DATA), "2805");
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
