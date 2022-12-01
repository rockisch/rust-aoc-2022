use std::{fs::File, io::Seek};

mod day01;

fn main() {
    let mut day01_file = File::open("inputs/day01.txt").unwrap();
    let result1 = day01::solve_1(&day01_file);
    day01_file.seek(std::io::SeekFrom::Start(0)).unwrap();
    let result2 = day01::solve_2(&day01_file);
    println!("Day 1: {}, {}", result1, result2);
}

#[cfg(test)]
mod tests {
    use std::io::Seek;

    use super::*;

    #[test]
    fn test_day01() {
        let mut day01_e_file = File::open("inputs/day01_e.txt").unwrap();
        let mut day01_file = File::open("inputs/day01.txt").unwrap();
        assert_eq!(day01::solve_1(&day01_e_file), "24000");
        assert_eq!(day01::solve_1(&day01_file), "70116");
        day01_e_file.seek(std::io::SeekFrom::Start(0)).unwrap();
        day01_file.seek(std::io::SeekFrom::Start(0)).unwrap();
        assert_eq!(day01::solve_2(&day01_e_file), "45000");
        assert_eq!(day01::solve_2(&day01_file), "206582");
    }
}
