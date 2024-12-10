//! # Advent of Code 2024 - Day 7
//!
//! This module contains the solution of the [sixth day's challenges](https://adventofcode.com/2024/day/6).
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


fn test(numbers: &[usize], target: usize) -> bool {
    std::iter::repeat_n(['+', '*'], numbers.len() - 1)
        .multi_cartesian_product()
        .any(|ops| {
            ops.iter()
                .zip(&numbers[1..])
                .fold(numbers[0], |acc, (op, &n)| match op {
                    '+' => acc + n,
                    _ => acc * n,
                })
                == target
        })
}

/// The solution to task 1 of day 7.
pub fn part_1(data: &[String]) -> usize {
    data.par_iter()
        .map(|s| parse(s))
        //.filter(|(target, numbers)|  test(numbers, *target, &operator_combis))
        .filter(|(target, numbers)| test(numbers, *target))
        .map(|(target, _)| target)
        .sum()
}

///// The solution to task 2 of day 7.
//pub fn part_2(data: &[String]) -> usize {
//3
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("190: 10 19"), (190, vec![10, 19]));
        assert_eq!(parse("3267: 81 40 27"), (3267, vec![81, 40, 27]));
    }

    //#[test]
    //fn test_test() {
    //assert!(test(&[10, 19], 190));
    //assert!(test(&[81, 40, 27], 3267));
    //assert!(test(&[11, 6, 16, 20], 292));
    //}

    #[test]
    fn test_operator_combis() {
        assert_eq!(
            construct_operator_combinations(3),
            vec![
                (2, vec![vec!['+'], vec!['*']].into_iter().collect()),
                (
                    3,
                    vec![
                        vec!['+', '+'],
                        vec!['+', '*'],
                        vec!['*', '+'],
                        vec!['*', '*']
                    ]
                    .into_iter()
                    .collect()
                )
            ]
            .into_iter()
            .collect()
        )
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
        assert_eq!(part_1(&data), 3749);
    }

    //#[test]
    //fn test_part_2() {
    //let data = [
    //];
    //assert_eq!(part_2(&data), 6);
    //}
}
