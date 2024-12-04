//! # Advent of Code 2024 - Day 4
//!
//! This module contains the solution of the [fourth day's challenges](https://adventofcode.com/2024/day/4).
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(data: &[String]) -> Self {
        Self {
            grid: data.iter().map(|s| s.chars().collect()).collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.grid[y][x]
    }

    fn char_positions(&self, target: char) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == target)
                    .map(move |(x, _)| (x, y))
            })
            .collect()
    }
}

/// The solution to task 1 of day 4.
pub fn day_04_1(data: &[String]) -> usize {
    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let grid = Grid::new(data);
    grid.char_positions('X')
        .into_iter()
        .map(|(x, y)| {
            directions
                .iter()
                .map(|(dx, dy)| {
                    (1..=3)
                        .scan((x as isize, y as isize), |(sx, sy), _| {
                            *sx += dx;
                            *sy += dy;
                            if *sx < 0
                                || *sy < 0
                                || *sx as usize >= grid.grid.len()
                                || *sy as usize >= grid.grid.len()
                            {
                                None
                            } else {
                                Some(grid.get(*sx as usize, *sy as usize))
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|v| *v == vec!['M', 'A', 'S'])
                .count()
        })
        .sum()
}

/// The solution to task 2 of day 4.
pub fn day_04_2(data: &[String]) -> usize {
    let grid = Grid::new(data);
    grid.char_positions('A')
        .iter()
        .filter(|&(x, y)| {
            *x > 0
                && *x < grid.grid[0].len() - 1
                && *y > 0
                && *y < grid.grid.len() - 1
                && ([('M', 'S'), ('S', 'M')]
                    .contains(&(grid.get(x - 1, y - 1), grid.get(x + 1, y + 1)))
                    && [('M', 'S'), ('S', 'M')]
                        .contains(&(grid.get(x - 1, y + 1), grid.get(x + 1, y - 1))))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_1() {
        let data = [
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];
        assert_eq!(day_04_1(&data), 18);
    }

    #[test]
    fn test_day_04_2() {
        let data = [
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];
        assert_eq!(day_04_2(&data), 9);
    }
}
