//! # Advent of Code 2024 - Day 11
//!
//! This module contains the solution of the [eleventh day's challenges](https://adventofcode.com/2024/day/11).
use itertools::Itertools;
use std::collections::HashMap as HM;

/// The solution to task 1 of day 11.
pub fn part_1_2(data: &str, blinks: usize) -> usize {
    let mut counts = data
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .counts();

    for _ in 1..=blinks {
        counts = counts
            .into_iter()
            .fold(HM::new(), |mut new_counts, (x, count)| {
                let str = format!("{x}");
                if x == 0 {
                    *new_counts.entry(1).or_insert(0) += count;
                } else if str.len() % 2 == 0 {
                    let i1 = str[0..str.len() / 2].parse::<usize>().unwrap();
                    let i2 = str[str.len() / 2..].parse::<usize>().unwrap();
                    *new_counts.entry(i1).or_insert(0) += count;
                    *new_counts.entry(i2).or_insert(0) += count;
                } else {
                    *new_counts.entry(x * 2024).or_insert(0) += count;
                }
                new_counts
            });
    }

    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_2() {
        let data = "125 17";
        assert_eq!(part_1_2(data, 6), 22);
        assert_eq!(part_1_2(data, 25), 55312);
    }
}
