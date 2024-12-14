//! # Advent of Code 2024 - Day 9
//!
//! This module contains the solution of the [ninth day's challenges](https://adventofcode.com/2024/day/9).
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Mem {
    Empty(usize),
    ID(usize),
}

fn parse_memory(data: &str, group_empty: bool) -> Vec<Mem> {
    data.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let n = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                std::iter::repeat(Mem::ID(i / 2)).take(n)
            } else if group_empty {
                std::iter::repeat(Mem::Empty(1)).take(n)
            } else {
                std::iter::repeat(Mem::Empty(n)).take(1)
            }
        })
        .collect()
}

/// The solution to task 1 of day 9.
pub fn part_1(data: &str) -> usize {
    // parse memory
    let parsed = parse_memory(data, false);

    let mut parsed_clone: VecDeque<usize> = parsed
        .clone()
        .into_iter()
        .filter_map(|x| match x {
            Mem::Empty(_) => None,
            Mem::ID(n) => Some(n),
        })
        .collect();

    let capacity = parsed.iter().filter(|x| matches!(x, Mem::ID(_))).count();
    let mut reordered = Vec::with_capacity(capacity);

    // reorder
    for elem in parsed {
        match elem {
            Mem::ID(n) => reordered.push(n),
            Mem::Empty(n) => {
                (0..n).for_each(|_| reordered.push(parsed_clone.pop_back().unwrap()));
            }
        };
        if reordered.len() == capacity {
            break;
        }
    }

    // compute result
    reordered.into_iter().enumerate().map(|(i, x)| i * x).sum()
}

/// The solution to task 2 of day 9.
pub fn part_2(data: &str) -> usize {
    // parse memory
    let mut parsed = parse_memory(data, true);

    // get the highest id
    let max_id = match parsed[parsed.len() - 1] {
        Mem::ID(n) => n,
        _ => panic!("error"),
    };

    for id in (0..=max_id).rev() {
        // find how many of these ids there are
        let (start, end) = (
            parsed.iter().position(|&p| p == Mem::ID(id)).unwrap(),
            parsed.len() - 1 - parsed.iter().rev().position(|&p| p == Mem::ID(id)).unwrap(),
        );
        let segment = end - start + 1;

        // find a block of empty spots to the left of that id to insert into
        if let Some(begin) = (1..start)
            .filter(|&i| parsed[i] == Mem::Empty(1))
            .scan(0, |len, i| {
                if parsed[i] == Mem::Empty(1) && parsed[i - 1] != Mem::Empty(1) {
                    *len = 1;
                } else if parsed[i] == Mem::Empty(1) {
                    *len += 1;
                } else {
                    *len = 0;
                }
                Some((*len, i))
            })
            .find(|&(len, _)| len == segment)
            .map(|(_, i)| i + 1 - segment)
        {
            // Perform the move
            for s in begin..begin + segment {
                parsed[s] = Mem::ID(id);
            }
            for s in start..=end {
                parsed[s] = Mem::Empty(1);
            }
        }
    }

    parsed
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| match p {
            Mem::Empty(_) => None,
            Mem::ID(n) => Some((i, n)),
        })
        .map(|(i, x)| i * x)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "2333133121414131402".to_string();
        assert_eq!(part_1(&data), 1928);
    }

    #[test]
    fn test_part_2() {
        let data = "2333133121414131402".to_string();
        assert_eq!(part_2(&data), 2858);
    }
}
