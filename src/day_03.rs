//! # Advent of Code 2024 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2024/day/3).
use regex::Regex;


/// The solution to task 1 of day 3.
pub fn day_03_1(data: &[String]) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    data.iter()
        .map(|s| re.captures_iter(s)
             .map(|c| c.get(1).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap() * c.get(2).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap())
             .sum::<u32>()
        )
        .sum()
}

/// The solution to task 2 of day 3.
//pub fn day_03_2(data: &[String]) -> usize {
    //3
//}

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
    }

    #[test]
    fn test_day_03_1() {
        let data = [
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];
        assert_eq!(day_03_1(&data), 161);
    }

    //#[test]
    //fn test_day_03_2() {
        //let data = [
        //];
        //assert_eq!(day_03_2(&data), 4);
    //}
}
