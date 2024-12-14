//! # Advent of Code 2024 - Day 8
//!
//! This module contains the solution of the [eigth day's challenges](https://adventofcode.com/2024/day/8).
use itertools::Itertools;
use std::collections::{HashMap as HM, HashSet as HS};

type Coord = (isize, isize);
type AntennaMap = HM<char, Vec<Coord>>;

fn parse(data: &[String]) -> AntennaMap {
    data.iter()
        .enumerate()
        .fold(AntennaMap::new(), |map, (y, line)| {
            line.chars().enumerate().fold(map, |mut map_loc, (x, c)| {
                if c != '.' {
                    map_loc.entry(c).or_default().push((x as isize, y as isize));
                }
                map_loc
            })
        })
}

/// The solution to task 1 of day 8.
pub fn part_1(data: &[String]) -> usize {
    parse(data)
        .into_iter()
        .fold(HS::new(), |mut antinodes, (_, antennas)| {
            antennas.into_iter().combinations(2).for_each(|v| {
                // for each combination of antennas, construct the 2 antinodes
                [(0, 1), (1, 0)]
                    .into_iter()
                    .map(|(a, b)| (2 * v[a].0 - v[b].0, 2 * v[a].1 - v[b].1))
                    // filter out the antinodes not within the map
                    .filter(|(x, y)| {
                        std::cmp::min(x, y) >= &0 && std::cmp::max(x, y) < &(data.len() as isize)
                    })
                    .for_each(|antinode| {
                        antinodes.insert(antinode);
                    });
            });
            antinodes
        })
        .len()
}

/// The solution to task 2 of day 8.
pub fn part_2(data: &[String]) -> usize {
    parse(data)
        .into_iter()
        .fold(HS::new(), |mut antinodes, (_, antennas)| {
            antennas.into_iter().combinations(2).for_each(|v| {
                let dir = (v[1].0 - v[0].0, v[1].1 - v[0].1);
                let gcd = num::integer::gcd(dir.0, dir.1);
                let dir = (dir.0 / gcd, dir.1 / gcd);
                let mut start = v[0];
                // Move into one direction along line connecting two antennas of same frequency
                while start.0 >= 0
                    && start.1 >= 0
                    && (start.0 as usize) < data.len()
                    && (start.1 as usize) < data.len()
                {
                    antinodes.insert(start);
                    start = (start.0 + dir.0, start.1 + dir.1);
                }
                // Move into other direction along line connecting two antennas of same frequency
                start = v[0];
                while start.0 >= 0
                    && start.1 >= 0
                    && (start.0 as usize) < data.len()
                    && (start.1 as usize) < data.len()
                {
                    antinodes.insert(start);
                    start = (start.0 - dir.0, start.1 - dir.1);
                }
            });

            antinodes
        })
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = [
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];

        let map = [
            ('0', vec![(8, 1), (5, 2), (7, 3), (4, 4)]),
            ('A', vec![(6, 5), (8, 8), (9, 9)]),
        ]
        .into_iter()
        .collect();
        assert_eq!(parse(&data), map);
    }

    #[test]
    fn test_part_1() {
        let data = [
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];
        assert_eq!(part_1(&data), 14);
    }

    #[test]
    fn test_part_2() {
        let data = [
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string(),
        ];
        assert_eq!(part_2(&data), 34);
    }
}
