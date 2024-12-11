//! # Advent of Code 2024 - Day 7
//!
//! This module contains the solution of the [seventh day's challenges](https://adventofcode.com/2024/day/7).
use itertools::Itertools;
use rayon::prelude::*;

fn parse(data: &str) -> (usize, Vec<usize>) {
    let mut split = data.split(':');
    (
        split.next().unwrap().trim().parse::<usize>().unwrap(),
        split
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
    )
}

fn test(numbers: &[usize], target: usize, operators: &[char]) -> bool {
    std::iter::repeat_n(operators, numbers.len() - 1)
        .multi_cartesian_product()
        .any(|ops| {
            ops.iter()
                .zip(&numbers[1..])
                .fold(numbers[0], |acc, (op, &n)| match op {
                    '+' => acc + n,
                    '*' => acc * n,
                    _ => acc * 10usize.pow(n.to_string().len() as u32) + n,
                })
                == target
        })
}

/// The solution to tasks 1 and 2 of day 7.
pub fn part_1_2(data: &[String], operators: &[char]) -> usize {
    data.par_iter()
        .map(|s| parse(s))
        .filter(|(target, numbers)| test(numbers, *target, operators))
        .map(|(target, _)| target)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("190: 10 19"), (190, vec![10, 19]));
        assert_eq!(parse("3267: 81 40 27"), (3267, vec![81, 40, 27]));
    }

    #[test]
    fn test_test() {
        assert!(test(&[10, 19], 190, &['+', '*']));
        assert!(test(&[81, 40, 27], 3267, &['+', '*']));
        assert!(test(&[11, 6, 16, 20], 292, &['+', '*']));
        assert!(!test(&[15, 6], 156, &['+', '*']));
        assert!(test(&[15, 6], 156, &['+', '*', '|']));
        assert!(test(&[17, 8, 14], 192, &['+', '*', '|']));
        assert!(test(&[6, 8, 6, 15], 7290, &['+', '*', '|']));
        assert!(test(&[53, 30], 5330, &['|']));
    }

    #[test]
    fn test_part_1() {
        let data = [
            "190: 10 19".to_string(),
            "3267: 81 40 27".to_string(),
            "83: 17 5".to_string(),
            "156: 15 6".to_string(),
            "7290: 6 8 6 15".to_string(),
            "161011: 16 10 13".to_string(),
            "192: 17 8 14".to_string(),
            "21037: 9 7 18 13".to_string(),
            "292: 11 6 16 20".to_string(),
        ];
        assert_eq!(part_1_2(&data, &['+', '*']), 3749);
    }

    #[test]
    fn test_part_2() {
        let data = [
            "190: 10 19".to_string(),
            "3267: 81 40 27".to_string(),
            "83: 17 5".to_string(),
            "156: 15 6".to_string(),
            "7290: 6 8 6 15".to_string(),
            "161011: 16 10 13".to_string(),
            "192: 17 8 14".to_string(),
            "21037: 9 7 18 13".to_string(),
            "292: 11 6 16 20".to_string(),
        ];
        assert_eq!(part_1_2(&data, &['+', '*', '|']), 11387);
    }
}
