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

    //#[test]
    //fn test_part_2() {
    //let data = [
    //];
    //assert_eq!(part_1_2(&data, &['+', '*', '|']), 11387);
    //}
}
