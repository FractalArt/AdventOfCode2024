//! # Advent of Code 2024 - Day 18
//!
//! This module contains the solution of the [eighteenth day's challenges](https://adventofcode.com/2024/day/18).
use std::collections::{HashSet as HS, VecDeque as VD};

fn simulate(corrupted: &HS<(isize, isize)>, max: isize) -> Option<usize> {
    let mut visited = HS::new();
    let mut todo: VD<_> = [((0, 0), 0)].into_iter().collect();

    while let Some(((x, y), steps)) = todo.pop_front() {
        if (x, y) == (max, max) {
            return Some(steps);
        } else if visited.contains(&(x, y)) {
            continue;
        } else {
            visited.insert((x, y));
        }

        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|&(xdx, ydy)| xdx >= 0 && ydy >= 0 && xdx <= max && ydy <= max)
            .filter(|&(xdx, ydy)| !corrupted.contains(&(xdx, ydy)))
            .for_each(|(xdx, ydy)| todo.push_back(((xdx, ydy), steps + 1)));
    }

    None
}

/// The solution to task 1 of day 18.
pub fn part_1(data: &[String], n_bytes: usize, max: isize) -> Option<usize> {
    let corrupted = data
        .iter()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<isize>().unwrap(),
                split.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .take(n_bytes)
        .collect::<HS<_>>();

    simulate(&corrupted, max)
}

/// The solution to task 2 of day 18.
pub fn part_2(data: &[String], max: isize) -> (isize, isize) {
    let corrupted = data
        .iter()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<isize>().unwrap(),
                split.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    (1..corrupted.len())
        .find_map(|i| {
            let hs = corrupted.iter().copied().take(i).collect();
            match simulate(&hs, max) {
                None => Some(corrupted[i - 1]),
                _ => None,
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = [
            "5,4".to_string(),
            "4,2".to_string(),
            "4,5".to_string(),
            "3,0".to_string(),
            "2,1".to_string(),
            "6,3".to_string(),
            "2,4".to_string(),
            "1,5".to_string(),
            "0,6".to_string(),
            "3,3".to_string(),
            "2,6".to_string(),
            "5,1".to_string(),
            "1,2".to_string(),
            "5,5".to_string(),
            "2,5".to_string(),
            "6,5".to_string(),
            "1,4".to_string(),
            "0,4".to_string(),
            "6,4".to_string(),
            "1,1".to_string(),
            "6,1".to_string(),
            "1,0".to_string(),
            "0,5".to_string(),
            "1,6".to_string(),
            "2,0".to_string(),
        ];
        assert_eq!(part_1(&data, 12, 6), Some(22));
    }

    #[test]
    fn test_part_2() {
        let data = [
            "5,4".to_string(),
            "4,2".to_string(),
            "4,5".to_string(),
            "3,0".to_string(),
            "2,1".to_string(),
            "6,3".to_string(),
            "2,4".to_string(),
            "1,5".to_string(),
            "0,6".to_string(),
            "3,3".to_string(),
            "2,6".to_string(),
            "5,1".to_string(),
            "1,2".to_string(),
            "5,5".to_string(),
            "2,5".to_string(),
            "6,5".to_string(),
            "1,4".to_string(),
            "0,4".to_string(),
            "6,4".to_string(),
            "1,1".to_string(),
            "6,1".to_string(),
            "1,0".to_string(),
            "0,5".to_string(),
            "1,6".to_string(),
            "2,0".to_string(),
        ];
        assert_eq!(part_2(&data, 6), (6, 1));
    }
}
