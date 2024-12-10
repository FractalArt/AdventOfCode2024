//! # Advent of Code 2024 - Day 6
//!
//! This module contains the solution of the [sixth day's challenges](https://adventofcode.com/2024/day/6).
use rayon::prelude::*;
use std::collections::HashSet as HS;

type Coord = (isize, isize);

#[derive(Default)]
struct Guard {
    coord: Coord,
    direction: Coord,
}

impl Guard {
    fn new(coord: Coord, direction: Coord) -> Self {
        Self { coord, direction }
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            _ => (0, -1),
        }
    }

    fn next(&self) -> Coord {
        (
            self.coord.0 + self.direction.0,
            self.coord.1 + self.direction.1,
        )
    }

    fn step(&mut self) {
        self.coord = self.next();
    }
}

/// Parse the data
fn parse_data(data: &[String]) -> (Vec<Coord>, Guard, Coord) {
    data.iter().enumerate().fold(
        (vec![], Guard::default(), (0, 0)),
        |(mut obstacles, mut guard, mut dim), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '^' => guard = Guard::new((x as isize, y as isize), (0, -1)),
                    'v' => guard = Guard::new((x as isize, y as isize), (0, 1)),
                    '>' => guard = Guard::new((x as isize, y as isize), (1, 0)),
                    '<' => guard = Guard::new((x as isize, y as isize), (-1, 0)),
                    '#' => obstacles.push((x as isize, y as isize)),
                    _ => {}
                }
                dim = (x as isize, y as isize);
            });
            (obstacles, guard, dim)
        },
    )
}

fn simulate(mut guard: Guard, obstacles: &[Coord], (dim_x, dim_y): Coord) -> HS<Coord> {
    let mut positions = [guard.coord].into_iter().collect::<HS<(_, _)>>();

    loop {
        if obstacles.contains(&guard.next()) {
            guard.turn();
        } else {
            guard.step();
            if guard.coord.0 >= 0
                && guard.coord.0 <= dim_x
                && guard.coord.1 >= 0
                && guard.coord.1 <= dim_y
            {
                positions.insert(guard.coord);
            } else {
                break positions;
            }
        }
    }
}

/// The solution to task 1 of day 6.
pub fn part_1(data: &[String]) -> usize {
    let (obstacles, guard, dim) = parse_data(data);
    simulate(guard, &obstacles, dim).len()
}

/// The solution to task 2 of day 6.
pub fn part_2(data: &[String]) -> usize {
    // Parse the data
    let (obstacles, guard, (dim_x, dim_y)) = parse_data(data);

    let start_pos = guard.coord;
    let start_dir = guard.direction;

    // Compute all the positions that the guard would visit, where an obstacle could be placed
    simulate(guard, &obstacles, (dim_x, dim_y))
        .into_par_iter()
        // filter out those that give loops
        .filter(|new_obs| {
            // simulate
            let mut guard = Guard::new(start_pos, start_dir);
            let mut visited = HS::with_capacity(5129);
            visited.insert((guard.coord, guard.direction));

            loop {
                if obstacles.contains(&guard.next()) || &guard.next() == new_obs {
                    guard.turn();
                } else {
                    guard.step();
                    if guard.coord.0 >= 0
                        && guard.coord.0 <= dim_x
                        && guard.coord.1 >= 0
                        && guard.coord.1 <= dim_y
                    {
                        let entry = (guard.coord, guard.direction);
                        if visited.contains(&entry) {
                            return true;
                        } else {
                            visited.insert(entry);
                        }
                    } else {
                        break false;
                    }
                }
            }
        })
        // count the ones that give loop
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = [
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];
        assert_eq!(part_1(&data), 41);
    }

    #[cfg(not(debug_assertions))]
    #[test]
    fn test_part_2() {
        let data = [
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];
        assert_eq!(part_2(&data), 6);
    }
}
