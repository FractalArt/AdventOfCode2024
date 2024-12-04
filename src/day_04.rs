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

    fn count_xmas(&self) -> usize {
        self.char_positions('X')
            .iter()
            .map(|&pos| {
                [
                    self.left_xmas(pos),
                    self.right_xmas(pos),
                    self.top_xmas(pos),
                    self.down_xmas(pos),
                    self.left_up(pos),
                    self.right_up(pos),
                    self.down_left(pos),
                    self.down_right(pos),
                ]
                .iter()
                .filter(|&x| *x)
                .count()
            })
            .sum()
    }

    fn left_xmas(&self, (x, y): (usize, usize)) -> bool {
        if x < 3 {
            false
        } else {
            self.grid[y][x - 3..=x - 1] == ['S', 'A', 'M']
        }
    }

    fn right_xmas(&self, (x, y): (usize, usize)) -> bool {
        if x >= self.grid[0].len() - 3 {
            false
        } else {
            self.grid[y][x + 1..=x + 3] == ['M', 'A', 'S']
        }
    }

    fn top_xmas(&self, (x, y): (usize, usize)) -> bool {
        if y < 3 {
            false
        } else {
            [
                self.grid[y - 1][x],
                self.grid[y - 2][x],
                self.grid[y - 3][x],
            ] == ['M', 'A', 'S']
        }
    }

    fn down_xmas(&self, (x, y): (usize, usize)) -> bool {
        if y >= self.grid.len() - 3 {
            false
        } else {
            [
                self.grid[y + 1][x],
                self.grid[y + 2][x],
                self.grid[y + 3][x],
            ] == ['M', 'A', 'S']
        }
    }

    fn left_up(&self, (x, y): (usize, usize)) -> bool {
        if y < 3 || x < 3 {
            false
        } else {
            [
                self.grid[y - 1][x - 1],
                self.grid[y - 2][x - 2],
                self.grid[y - 3][x - 3],
            ] == ['M', 'A', 'S']
        }
    }

    fn right_up(&self, (x, y): (usize, usize)) -> bool {
        if y < 3 || x >= self.grid[0].len() - 3 {
            false
        } else {
            [
                self.grid[y - 1][x + 1],
                self.grid[y - 2][x + 2],
                self.grid[y - 3][x + 3],
            ] == ['M', 'A', 'S']
        }
    }

    fn down_left(&self, (x, y): (usize, usize)) -> bool {
        if y >= self.grid.len() - 3 || x < 3 {
            false
        } else {
            [
                self.grid[y + 1][x - 1],
                self.grid[y + 2][x - 2],
                self.grid[y + 3][x - 3],
            ] == ['M', 'A', 'S']
        }
    }

    fn down_right(&self, (x, y): (usize, usize)) -> bool {
        if y >= self.grid.len() - 3 || x >= self.grid[0].len() - 3 {
            false
        } else {
            [
                self.grid[y + 1][x + 1],
                self.grid[y + 2][x + 2],
                self.grid[y + 3][x + 3],
            ] == ['M', 'A', 'S']
        }
    }
}

/// The solution to task 1 of day 4.
pub fn day_04_1(data: &[String]) -> usize {
    Grid::new(data).count_xmas()
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
                    .contains(&(grid.grid[y - 1][x - 1], grid.grid[y + 1][x + 1]))
                    && [('M', 'S'), ('S', 'M')]
                        .contains(&(grid.grid[y + 1][x - 1], grid.grid[y - 1][x + 1])))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_04_grid() {
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

        let grid = Grid::new(&data);
        let xs = grid.char_positions('X');
        assert_eq!(
            vec![
                (4, 0),
                (5, 0),
                (4, 1),
                (2, 2),
                (4, 2),
                (9, 3),
                (0, 4),
                (6, 4),
                (0, 5),
                (1, 5),
                (5, 5),
                (6, 5),
                (7, 6),
                (2, 7),
                (5, 8),
                (1, 9),
                (3, 9),
                (5, 9),
                (9, 9)
            ],
            xs
        );

        // left
        assert!(grid.left_xmas((4, 1)));
        assert!(!grid.left_xmas((4, 0)));
        assert!(!grid.left_xmas((0, 0)));

        // right
        assert!(grid.right_xmas((5, 0)));
        assert!(!grid.right_xmas((9, 9)));

        // top
        assert!(!grid.top_xmas((0, 0)));
        assert!(grid.top_xmas((9, 9)));

        // down
        assert!(!grid.down_xmas((0, 0)));
        assert!(grid.down_xmas((9, 3)));

        // left up
        assert!(grid.left_up((9, 9)));
        assert!(!grid.left_up((2, 2)));

        // right up
        assert!(!grid.right_up((7, 7)));
        assert!(grid.right_up((1, 9)));

        // down left
        assert!(!grid.down_left((7, 7)));
        assert!(grid.down_left((9, 3)));

        // down right
        assert!(!grid.down_right((7, 7)));
        assert!(grid.down_right((4, 0)));
    }

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
