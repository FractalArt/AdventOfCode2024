//! # Advent of Code 2024 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2024/day/3).
use regex::Regex;

/// The solution to task 1 of day 3.
pub fn part_1(mem: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(mem)
        .map(|c| {
            c.get(1)
                .map(|m| m.as_str())
                .unwrap()
                .parse::<u32>()
                .unwrap()
                * c.get(2)
                    .map(|m| m.as_str())
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
        })
        .sum::<u32>()
}

/// The solution to task 2 of day 3.
pub fn part_2(mem: &str) -> u32 {
    let re = Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)|(?P<dont>don't\(\))|(?P<do>do\(\))")
        .unwrap();
    re.captures_iter(mem)
        .fold((true, 0), |(enabled, sum), c| {
            if c.name("do").map(|m| m.as_str()).is_some() {
                (true, sum)
            } else if c.name("dont").map(|m| m.as_str()).is_some() {
                (false, sum)
            } else if enabled {
                (
                    enabled,
                    c.name("x")
                        .map(|m| m.as_str())
                        .unwrap()
                        .parse::<u32>()
                        .unwrap()
                        * c.name("y")
                            .map(|m| m.as_str())
                            .unwrap()
                            .parse::<u32>()
                            .unwrap()
                        + sum,
                )
            } else {
                (enabled, sum)
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        assert!(re.is_match("mul(1,2)"));
        assert!(re.is_match("mul(1,12)"));
        assert!(re.is_match("mul(1,123)"));
        assert!(re.is_match("mul(12,12)"));
        assert!(re.is_match("mul(123,12)"));
        assert!(re.is_match("mul(123,1)"));
        assert!(!re.is_match("mul(1234,1)"));
        let re =
            Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)|(?P<dont>don't\(\))|(?P<do>do\(\))")
                .unwrap();
        assert!(re.is_match("don't()"));
        assert!(!re.is_match("don't"));
        assert!(re.is_match("do()"));
        assert!(!re.is_match("do"));
    }

    #[test]
    fn test_part_1() {
        let data =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(part_1(&data), 161);
    }

    #[test]
    fn test_part_2() {
        let data =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(part_2(&data), 48);
    }
}
