#![feature(test)]
#![feature(array_chunks)]
extern crate test;

pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
pub const DATA: &[u8] = include_bytes!("data.txt");

fn main() {
    let r1 = solve_1(DATA);
    let r2 = solve_2(DATA);
    println!("RESULTS: {r1} {r2}");
}

pub fn solve_1(input: &[u8]) -> String {
    input.split(|&v| v == b'\n')
        .filter(|v| !v.is_empty())
        .map(|l| (l[0] - b'A', l[2] - b'X'))
        .map(|(a, b)| ((4 + b - a) % 3 * 3 + b + 1) as u16)
        .sum::<u16>()
        .to_string()
}

pub fn solve_2(input: &[u8]) -> String {
    input.split(|&v| v == b'\n')
        .filter(|v| !v.is_empty())
        .map(|l| (l[0] - b'A', l[2] - b'X'))
        .map(|(a, b)| ((2 + a + b) % 3 + b * 3 + 1) as u16)
        .sum::<u16>()
        .to_string()
}

pub fn solve_1_steroids(input: &[u8]) -> String {
    input.array_chunks::<4>().map(|line| {
        let ab = unsafe { line.get_unchecked(0) };
        let cd = unsafe { line.get_unchecked(2) };
        let a = ab >> 1;
        let b = ab;
        let c = cd >> 1;
        let d = cd;
        let r = (((!a & d) | (!b & c)) & 0b1) << 3
            | (((!b & d) | (!a & !c & !d) | (a & b & !d)) & 0b1) << 2
            | (((a & b) | (!a & c)) & 0b1) << 1
            | (((!a & c) | !b | (a & !c & !d)) & 0b1);
        r as u32
    }).sum::<u32>().to_string()
}

pub fn solve_2_steroids(input: &[u8]) -> String {
    input.array_chunks::<4>().map(|line| {
        let ab = unsafe { line.get_unchecked(0) };
        let cd = unsafe { line.get_unchecked(2) };
        let a = ab >> 1;
        let b = ab;
        let c = cd >> 1;
        let d = cd;
        let r = (((!a & c) | (!b & c)) & 0b1) << 3
            | ((d | (a & b & c)) & 0b1) << 2
            | (((a & b) | (!a & !c & !d)) & 0b1) << 1
            | (((!a & !c & !d) | !b | (a & c)) & 0b1);
        r as u32
    }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve_1(EXAMPLE), "15");
        assert_eq!(solve_1(DATA), "12645");
        assert_eq!(solve_1_steroids(EXAMPLE), "15");
        assert_eq!(solve_1_steroids(DATA), "12645");
        assert_eq!(solve_2(EXAMPLE), "12");
        assert_eq!(solve_2(DATA), "11756");
        assert_eq!(solve_2_steroids(EXAMPLE), "12");
        assert_eq!(solve_2_steroids(DATA), "11756");
    }

    #[bench]
    fn bench_1(b: &mut Bencher) {
        b.iter(|| solve_1(black_box(DATA)));
    }

    #[bench]
    fn bench_2(b: &mut Bencher) {
        b.iter(|| solve_2(black_box(DATA)));
    }

    #[bench]
    fn bench_1_steroids(b: &mut Bencher) {
        b.iter(|| solve_1_steroids(black_box(DATA)));
    }

    #[bench]
    fn bench_2_steroids(b: &mut Bencher) {
        b.iter(|| solve_2_steroids(black_box(DATA)));
    }
}
