//! # Advent of Code 2024 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2024/day/2).
use itertools::Itertools;
use std::hash::Hash;

#[derive(PartialEq, Hash, Eq)]
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

#[derive(Debug, PartialEq)]
enum Rep {
    Valid,
    Invalid,
    PotentiallyFixable,
}

fn validate_report(report: &[i32]) -> Rep {
    let  counts = report.iter()
        .tuple_windows()
        .map(|(x, y)| match x-y {
            d if d.abs() > 0 && d.abs() < 4 && d < 0 => Report::ValidDecreasing,
            d if d.abs() > 0 && d.abs() < 4 && d > 0 => Report::ValidIncreasing,
            _ => Report::Invalid
        })
        .counts();

        match (*counts.get(&Report::Invalid).unwrap_or(&0), *counts.get(&Report::ValidIncreasing).unwrap_or(&0), *counts.get(&Report::ValidDecreasing).unwrap_or(&0)) {
            (0, 0, _d) => Rep::Valid,
            (0, _d, 0) => Rep::Valid,
            (1, 0, _d) => Rep::PotentiallyFixable,
            (1, _d, 0) => Rep::PotentiallyFixable,
            (0, 1, _d) => Rep::PotentiallyFixable,
            (0, _d, 1) => Rep::PotentiallyFixable,
            _ => Rep::Invalid,

        }


}


fn try_fix(report: &[i32]) -> bool {
    for i in 0..report.len() {
        let mut clone: Vec<i32> = report.to_vec();
        clone.remove(i);
        if validate_report(&clone) == Rep::Valid {
            return true
        }

    }
    false
}

/// The solution to task 2 of day 2.
pub fn day_02_2(data: &[String]) -> usize {
    data.iter()
        .map(|report| {
            let report: Vec<_>  = report.split_whitespace()
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect();
            match validate_report(&report) {
                Rep::Valid => true,
                Rep::Invalid => try_fix(&report),
                Rep::PotentiallyFixable => try_fix(&report)
            }
        })
        .filter(|&r| r)
        .count()
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

    #[test]
    fn test_validate_report() {
        assert_eq!(validate_report(&[7, 6, 4, 2, 1]), Rep::Valid);
        assert_eq!(validate_report(&[1, 2, 7, 8, 9]), Rep::PotentiallyFixable);
        assert_eq!(validate_report(&[9, 7, 6, 2, 1]), Rep::PotentiallyFixable);
        assert_eq!(validate_report(&[1, 3, 2, 4, 5]), Rep::PotentiallyFixable);
        assert_eq!(validate_report(&[8, 6, 4, 4, 1]), Rep::PotentiallyFixable);
        assert_eq!(validate_report(&[1, 3, 6, 7, 9]), Rep::Valid);
    }

    #[test]
    fn test_try_fix() {
        assert!(!try_fix(&[1, 2, 7, 8, 9]));
        assert!(!try_fix(&[9, 7, 6, 2, 1]));
        assert!(try_fix(&[1, 3, 2, 4, 5]));
        assert!(try_fix(&[8, 6, 4, 4, 1]));
    }
    
    #[test]
    fn test_day_02_2() {
        let data = [
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(day_02_2(&data), 4);
    }

}
