//! # Advent of Code 2024 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2024/day/2).
use itertools::Itertools;
use std::hash::Hash;

#[derive(PartialEq, Hash, Eq)]
enum Diff {
    ValidIncreasing,
    ValidDecreasing,
    Invalid,
}

fn valid_report(report: &[i32]) -> bool {
    let counts = report
        .iter()
        .tuple_windows()
        .map(|(x, y)| match x - y {
            d if d.abs() > 0 && d.abs() < 4 && d < 0 => Diff::ValidDecreasing,
            d if d.abs() > 0 && d.abs() < 4 && d > 0 => Diff::ValidIncreasing,
            _ => Diff::Invalid,
        })
        .counts();

    matches!(
        (
            *counts.get(&Diff::Invalid).unwrap_or(&0),
            *counts.get(&Diff::ValidIncreasing).unwrap_or(&0),
            *counts.get(&Diff::ValidDecreasing).unwrap_or(&0)
        ),
        (0, 0, _d) | (0, _d, 0)
    )
}

fn try_fix(report: &[i32]) -> bool {
    report
        .iter()
        .enumerate()
        .any(|(i, _)| valid_report(&[&report[..i], &report[i + 1..]].concat()))
}

/// The solution to task 1 of day 2.
pub fn part_1(data: &[String]) -> usize {
    data.iter()
        .map(|report| {
            valid_report(
                &report
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|&r| r)
        .count()
}

/// The solution to task 2 of day 2.
pub fn part_2(data: &[String]) -> usize {
    data.iter()
        .map(|report| {
            let report: Vec<_> = report
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            match valid_report(&report) {
                true => true,
                false => try_fix(&report),
            }
        })
        .filter(|&r| r)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = [
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(part_1(&data), 2);
    }

    #[test]
    fn test_valid_report() {
        assert!(valid_report(&[7, 6, 4, 2, 1]));
        assert!(!valid_report(&[1, 2, 7, 8, 9]));
        assert!(!valid_report(&[9, 7, 6, 2, 1]));
        assert!(!valid_report(&[1, 3, 2, 4, 5]));
        assert!(!valid_report(&[8, 6, 4, 4, 1]));
        assert!(valid_report(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_try_fix() {
        assert!(!try_fix(&[1, 2, 7, 8, 9]));
        assert!(!try_fix(&[9, 7, 6, 2, 1]));
        assert!(try_fix(&[1, 3, 2, 4, 5]));
        assert!(try_fix(&[8, 6, 4, 4, 1]));
    }

    #[test]
    fn test_part_2() {
        let data = [
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(part_2(&data), 4);
    }
}
