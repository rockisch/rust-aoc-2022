use std::{
    fs,
    io::{self, BufRead},
};

use crate::utils;

pub fn solve_file<const N: usize>(input_file: &fs::File) -> String {
    let lines = io::BufReader::new(input_file).lines();
    let mut fixed_heap = utils::FixedBinaryHeap::<u32, N>::new();
    let mut acc = 0;
    for line in lines {
        let line = unsafe { line.unwrap_unchecked() };
        if line.is_empty() {
            fixed_heap.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }
    fixed_heap.push(acc);
    fixed_heap.into_iter().sum::<u32>().to_string()
}

pub fn solve_str<const N: usize>(input_str: &str) -> String {
    let lines = input_str.lines();
    let mut fixed_heap = utils::FixedBinaryHeap::<u32, N>::new();
    let mut acc = 0;
    for line in lines {
        if line.is_empty() {
            fixed_heap.push(acc);
            acc = 0;
        } else {
            acc += line.parse::<u32>().unwrap();
        }
    }
    fixed_heap.push(acc);
    fixed_heap.into_iter().sum::<u32>().to_string()
}
