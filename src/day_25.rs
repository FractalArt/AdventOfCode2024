//! # Advent of Code 2024 - Day 25
//!
//! This module contains the solution of the [twentyfifth day's challenges](https://adventofcode.com/2024/day/25).
use ndarray::{Array2, Axis};

/// The solution to task 1 of day 25.
pub fn part_1(data: &str) -> usize {
    let (locks, keys) =
        data.split("\n\n")
            .fold((vec![], vec![]), |(mut locks, mut keys), candidate| {
                let array = Array2::from_shape_vec(
                    (7, 5),
                    candidate
                        .split("\n")
                        .flat_map(|line| line.chars())
                        .map(|c| if c == '#' { 1 } else { 0 })
                        .collect(),
                )
                .unwrap();
                if array.row(0).iter().all(|&x| x == 1) {
                    locks.push(array.sum_axis(Axis(0)) - 1)
                } else {
                    keys.push(array.sum_axis(Axis(0)) - 1)
                }
                (locks, keys)
            });

    locks
        .iter()
        .map(|l| {
            keys.iter()
                .filter(|k| l.iter().zip(k.iter()).map(|(&l, &k)| l + k).all(|x| x < 6))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(part_1(data), 3);
    }
}
