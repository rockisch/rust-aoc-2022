use std::{fs, io::{self, BufRead}};

pub fn solve_1(input_file: &fs::File) -> String {
    let lines = io::BufReader::new(input_file).lines();
    let mut max = 0;
    let mut acc = 0;
    for line in lines {
        let line = line.unwrap();
        if line.len() > 0 {
            acc += line.parse::<u32>().unwrap();
        } else {
            if acc > max {
                max = acc;
            }
            acc = 0;
        }
    }
    max.to_string()
}

pub fn solve_2(input_file: &fs::File) -> String {
    let lines = io::BufReader::new(input_file).lines();
    let mut maxes = [0, 0, 0];
    let mut acc = 0;
    let mut set_max = |val| {
        if val > maxes[0] {
            if val > maxes[1] {
                if val > maxes[2] {
                    maxes[0] = maxes[1];
                    maxes[1] = maxes[2];
                    maxes[2] = val;
                } else {
                    maxes[0] = maxes[1];
                    maxes[1] = val;
                }
            } else {
                maxes[0] = val;
            }
        }
    };
    for line in lines {
        let line = unsafe { line.unwrap_unchecked() };
        if line.len() > 0 {
            acc += unsafe { line.parse::<u32>().unwrap_unchecked() };
        } else {
            set_max(acc);
            acc = 0;
        }
    }
    set_max(acc);
    maxes.into_iter().sum::<u32>().to_string()
}
