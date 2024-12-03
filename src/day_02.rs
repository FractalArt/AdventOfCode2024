//! # Advent of Code 2024 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2024/day/2).
use itertools::Itertools;

#[derive(PartialEq)]
enum Report {
    ValidIncreasing,
    ValidDecreasing,
    Invalid,
}

/// The solution to task 1 of day 2.
pub fn day_02_1(data: &[String]) -> usize {
    data.iter()
        .map(|report| report.split_whitespace()
                            .map(|x| x.parse::<i32>().unwrap())
                            .tuple_windows()
                            .map(|(x, y)| match x-y {
                                d if d.abs() > 0 && d.abs() < 4 && d < 0 => Report::ValidDecreasing,
                                d if d.abs() > 0 && d.abs() < 4 && d > 0 => Report::ValidIncreasing,
                                _ => Report::Invalid
                            })
                            .all_equal()

            )
        .filter(|&r| r)
        .count()
}

/// The solution to task 2 of day 2.
pub fn day_02_2(data: &[String]) -> u32 {
    2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_1() {
        let data = [
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(day_02_1(&data), 2);
    }

    //#[test]
    //fn test_day_01_2() {
        //let data = [
            //"3   4".to_string(),
            //"4   3".to_string(),
            //"2   5".to_string(),
            //"1   3".to_string(),
            //"3   9".to_string(),
            //"3   3".to_string(),
        //];
        //assert_eq!(day_01_2(&data), 31);
    //}

}
