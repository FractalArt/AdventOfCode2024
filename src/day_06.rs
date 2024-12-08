//! # Advent of Code 2024 - Day 6
//!
//! This module contains the solution of the [sixth day's challenges](https://adventofcode.com/2024/day/6).
use std::collections::HashSet as HS;
use rayon::prelude::*;

type Coord = (isize, isize);

#[derive(Default)]
struct Guard {
    coord: Coord,
    direction: Coord,
}

impl Guard {
    fn new(coord: Coord, symbol: char) -> Self {
        match symbol {
            '^' => Self {
                coord,
                direction: (0, -1),
            },
            'v' => Self {
                coord,
                direction: (0, 1),
            },
            '>' => Self {
                coord,
                direction: (1, 0),
            },
            _ => Self {
                coord,
                direction: (-1, 0),
            },
        }
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

/// The solution to task 1 of day 6.
pub fn part_1(data: &[String]) -> usize {
    let (obstacles, mut guard, (dim_x, dim_y)) = data.iter().enumerate().fold(
        (vec![], Guard::default(), (0, 0)),
        |(mut obstacles, mut guard, mut dim), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '^' | 'v' | '>' | '<' => {
                        guard = Guard::new((x as isize, y as isize), c);
                    }
                    '#' => obstacles.push((x as isize, y as isize)),
                    _ => {}
                }
                dim = (x as isize, y as isize);
            });
            (obstacles, guard, dim)
        },
    );

    let mut positions = [guard.coord].into_iter().collect::<HS<(_,_)>>();

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
                break positions.len();
            }
        }
    }

}

/// The solution to task 2 of day 6.
pub fn part_2(data: &[String]) -> usize {
    
    // Parse the data
    let (obstacles, mut guard, (dim_x, dim_y)) = data.iter().enumerate().fold(
        (vec![], Guard::default(), (0, 0)),
        |(mut obstacles, mut guard, mut dim), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '^' | 'v' | '>' | '<' => {
                        guard = Guard::new((x as isize, y as isize), c);
                    }
                    '#' => obstacles.push((x as isize, y as isize)),
                    _ => {}
                }
                dim = (x as isize, y as isize);
            });
            (obstacles, guard, dim)
        },
    );

    let start_pos = guard.coord;
    let start_dir = guard.direction;

    // Compute all the positions that the guard would visit

    let mut positions = [guard.coord].into_iter().collect::<HS<(_,_)>>();

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
                break;
            }
        }
    }
    // Now try to place an obstacle in every place the guard visited and check if it leads to a
    // loop
    
    // loop over all new obstacle positions
    positions.into_par_iter()
        // filter out those that give loops
        .filter(|new_obs| {
            // simulate
            let mut guard = Guard::default();
            guard.coord = start_pos;
            guard.direction = start_dir;

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
                        visited.insert((guard.coord, guard.direction));
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
