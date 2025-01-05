//! # Advent of Code 2024 - Day 22
//!
//! This module contains the solution of the [twentysecond day's challenges](https://adventofcode.com/2024/day/22).
use rayon::prelude::*;

fn next(value: usize) -> usize {
        let mut value = ((value * 64) ^ value) % 16777216;
        value = ((value / 32) ^ value) % 16777216;
        ((value * 2048) ^ value) % 16777216
}

/// The solution to task 1 of day 22.
pub fn part_1(data: &[String], iter: usize) -> usize {
        data.par_iter()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .map(|x| (0..iter).fold(x, |accum, _| next(accum)))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_number() {
        let mut secret = 123;
        secret = next(secret);
        assert_eq!(secret, 15887950);

        let mut secret = 123;

        for _ in 0..10 {
            secret = next(secret);
        }

        assert_eq!(secret, 5908254);

        let mut secret = 1;

        for _ in 0..2000 {
            secret = next(secret);
        }

        assert_eq!(secret, 8685429);

        let mut secret = 10;

        for _ in 0..2000 {
            secret = next(secret);
        }

        assert_eq!(secret, 4700978);
    }



    #[test]
    fn test_part_1() {
        let data = [
            "1".to_string(),
            "10".to_string(),
            "100".to_string(),
            "2024".to_string(),
        ];
        assert_eq!(part_1(&data, 2000), 37327623);
    }

}
