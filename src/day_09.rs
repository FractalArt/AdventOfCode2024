//! # Advent of Code 2024 - Day 9
//!
//! This module contains the solution of the [ninth day's challenges](https://adventofcode.com/2024/day/9).
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mem {
    Empty(usize),
    ID(usize),
}

/// The solution to task 1 of day 9.
pub fn part_1(data: &str) -> usize {
    // parse memory
    let parsed: Vec<Mem> = data
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let n = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                std::iter::repeat(Mem::ID(i / 2)).take(n)
            } else {
                std::iter::repeat(Mem::Empty(n)).take(1)
            }
        })
        .collect();

    let mut parsed_clone: VecDeque<usize> = parsed.clone().into_iter().filter_map(|x| match x {
        Mem::Empty(_) => None,
        Mem::ID(n) => Some(n),
    }).collect();

    let capacity = parsed
        .iter()
        .filter(|x| matches!(x, Mem::ID(_)))
        .count();
    let mut reordered = Vec::with_capacity(capacity);

    // reorder
    for elem in parsed {
        match elem {
            Mem::ID(n) => reordered.push(n),
            Mem::Empty(n) => {(0..n).for_each(|_| reordered.push(parsed_clone.pop_back().unwrap()));}
        };
        if reordered.len() == capacity {
            break;
        }
    }

    // compute result
    reordered.into_iter()
        .enumerate()
        .map(|(i, x)| i * x)
        .sum()
}

///// The solution to task 2 of day 9.
//pub fn part_2(data: &str) -> usize {
    //3
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "2333133121414131402".to_string();
        assert_eq!(part_1(&data), 1928);
    }

    //#[test]
    //fn test_part_2() {
        //let data = "2333133121414131402".to_string();
        //assert_eq!(part_2(&data), 34);
    //}
}
