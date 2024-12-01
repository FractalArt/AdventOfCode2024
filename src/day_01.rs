//! # Advent of Code 2024 - Day 1
//!
//! This module contains the solution of the [first day's challenges](https://adventofcode.com/2024/day/1).
use std::collections::HashMap as HM;

/// The solution to task 1 of day 1.
pub fn day_01_1(data: &[String]) -> i32 {
    let (mut v1, mut v2) = data.iter()
        .map(|s| s.split_once("   ").unwrap())
        .map(|(x1, x2)| (x1.parse::<i32>().unwrap(), x2.parse::<i32>().unwrap()))
        .fold((Vec::with_capacity(data.len()), Vec::with_capacity(data.len())), |(mut v1, mut v2), (x1, x2)| {v1.push(x1); v2.push(x2); (v1, v2)});
    v1.sort();
    v2.sort();
    v1.into_iter().zip(v2)
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum()
}

/// The solution to task 2 of day 1.
pub fn day_01_2(data: &[String]) -> u32 {
    let (v, hm) = data.iter()
        .map(|s| s.split_once("   ").unwrap())
        .map(|(x1, x2)| (x1.parse::<u32>().unwrap(), x2.parse::<u32>().unwrap()))
            .fold((Vec::with_capacity(data.len()), HM::<u32, u32>::new()), |(mut v, mut hm), (x1, x2)| {
                hm.entry(x2).and_modify(|counter| *counter += 1).or_insert(1);
                v.push(x1);
                (v, hm)
            });
    v.into_iter()
        .map(|x| x * hm.get(&x).unwrap_or(&0))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_1() {
        let data = [
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        assert_eq!(day_01_1(&data), 11);
    }

    #[test]
    fn test_day_01_2() {
        let data = [
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];
        assert_eq!(day_01_2(&data), 31);
    }

}
