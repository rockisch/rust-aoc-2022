#![feature(test)]
extern crate test;

pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
pub const DATA: &[u8] = include_bytes!("data.txt");

fn main() {
    let r1 = solve_1(DATA);
    let r2 = solve_2(DATA);
    println!("RESULTS: {r1} {r2}");
}

fn solve_1(input: &[u8]) -> String {
    "".to_owned()
}

fn solve_2(input: &[u8]) -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};
    use super::*;

    #[test]
    fn test() {
        assert_eq!(solve_1(EXAMPLE), "");
        assert_eq!(solve_1(DATA), "");
        assert_eq!(solve_2(EXAMPLE), "");
        assert_eq!(solve_2(DATA), "");
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
