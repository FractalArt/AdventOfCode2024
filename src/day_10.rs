//! # Advent of Code 2024 - Day 10
//!
//! This module contains the solution of the [tenth day's challenges](https://adventofcode.com/2024/day/10).
use std::cmp::{max, min};

fn parse_map(data: &[String]) -> Vec<Vec<i32>> {
    data.iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn score(trailhead: (isize, isize), map: &[Vec<i32>], count_paths: bool) -> usize {
    let mut todo = std::collections::VecDeque::new();
    todo.push_back(trailhead);
    let mut reached_destinations = std::collections::HashSet::new();
    let mut path_count = 0;

    while let Some((x, y)) = todo.pop_back() {
        // store the number in the position
        let n = map[y as usize][x as usize];

        if n == 9 {
            reached_destinations.insert((x, y));
            path_count += 1;
            continue;
        }

        // look in the surroundings for valid next steps
        [(0, 1), (1, 0), (-1, 0), (0, -1)]
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(xx, yy)| min(*xx, *yy) >= 0 && max(*xx, *yy) < map.len() as isize)
            .filter(|&(x, y)| map[y as usize][x as usize] == n + 1)
            .for_each(|step| todo.push_back(step));
    }

    if count_paths {
        path_count
    } else {
        reached_destinations.len()
    }
}

/// The solution to task 1 of day 10.
pub fn part_1(data: &[String]) -> usize {
    // parse the map
    let map = parse_map(data);
    // identify the trailheads
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &n)| n == 0)
                .map(move |(x, _)| (x as isize, y as isize))
        })
        // for each trailhead compute the score by counting the number of 9s that can be reached
        .map(|th| score(th, &map, false))
        .sum()
}

/// The solution to task 2 of day 9.
pub fn part_2(data: &[String]) -> usize {
    // parse the map
    let map = parse_map(data);
    // identify the trailheads
    map.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &n)| n == 0)
                .map(move |(x, _)| (x as isize, y as isize))
        })
        // for each trailhead identify the score by counting the number of trails
        .map(|th| score(th, &map, true))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = [
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];
        let map = parse_map(&data);
        assert_eq!(map[0][0], 8);
        assert_eq!(map[0][1], 9);
        assert_eq!(map[1][0], 7);
    }

    #[test]
    fn test_score() {
        let data = [
            "9990999".to_string(),
            "9991999".to_string(),
            "9992999".to_string(),
            "6543456".to_string(),
            "7999997".to_string(),
            "8099908".to_string(),
            "9099909".to_string(),
        ];
        assert_eq!(score((3, 0), &parse_map(&data), false), 2);
    }

    #[test]
    fn test_part_1() {
        let data = [
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];
        assert_eq!(part_1(&data), 36);
    }

    #[test]
    fn test_part_2() {
        let data = [
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ];
        assert_eq!(part_2(&data), 81);
    }
}
