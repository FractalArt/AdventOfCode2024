//! # Advent of Code 2024 - Day 14
//!
//! This module contains the solution of the [fourteenth day's challenges](https://adventofcode.com/2024/day/14).

type PosVel = ((isize, isize), (isize, isize));

fn simulate(posvel: Vec<PosVel>, xdim: isize, ydim: isize) -> Vec<PosVel> {
    posvel
        .into_iter()
        .map(|(pos, vel)| {
            (
                (
                    (pos.0 + vel.0).rem_euclid(xdim),
                    (pos.1 + vel.1).rem_euclid(ydim),
                ),
                vel,
            )
        })
        .collect()
}

fn parse(data: &[String]) -> Vec<PosVel> {
    data.iter()
        .map(|line| line.split_whitespace())
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(pos, vel)| (pos[2..].split(','), vel[2..].split(',')))
        .map(|(mut p_split, mut v_split)| {
            (
                (
                    p_split.next().unwrap().parse::<isize>().unwrap(),
                    p_split.next().unwrap().parse::<isize>().unwrap(),
                ),
                (
                    v_split.next().unwrap().parse::<isize>().unwrap(),
                    v_split.next().unwrap().parse::<isize>().unwrap(),
                ),
            )
        })
        .collect()
}

/// The solution to task 1 of day 14.
pub fn part_1(data: &[String], iter: usize, xdim: isize, ydim: isize) -> usize {
    let mut posvel = parse(data);

    for _ in 0..iter {
        posvel = simulate(posvel, xdim, ydim);
    }

    posvel
        .into_iter()
        .map(|(pos, _)| pos)
        .fold([0, 0, 0, 0], |v, (x, y)| match (x, y) {
            _ if x < xdim / 2 && y < ydim / 2 => [v[0] + 1, v[1], v[2], v[3]],
            _ if x > xdim / 2 && y < ydim / 2 => [v[0], v[1] + 1, v[2], v[3]],
            _ if x < xdim / 2 && y > ydim / 2 => [v[0], v[1], v[2] + 1, v[3]],
            _ if x > xdim / 2 && y > ydim / 2 => [v[0], v[1], v[2], v[3] + 1],
            _ => v,
        })
        .into_iter()
        .product()
}

/// The solution to task 2 of day 14.
pub fn part_2(data: &[String], xdim: isize, ydim: isize) -> Option<usize> {
    let mut posvel = parse(data);

    for i in 1..10000 {
        posvel = simulate(posvel, xdim, ydim);
        let coords: std::collections::HashSet<(isize, isize)> =
            posvel.iter().map(|&(x, _)| x).collect();

        if (0..ydim).any(|y| {
            (0..xdim)
                .map(|x| if coords.contains(&(x, y)) { 'X' } else { '.' })
                .collect::<String>()
                .contains("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
        }) {
            //draw(&posvel, xdim, ydim);
            return Some(i);
        }
    }

    None
}

//fn draw(vec: &[PosVel], xdim: isize, ydim: isize) {
//let coords: std::collections::HashSet<(isize, isize)> = vec.iter().map(|&(x, _)| x).collect();

//for y in 0..ydim {
//let tline: String = (0..xdim)
//.map(|x| if coords.contains(&(x, y)) { 'X' } else { '.' })
//.collect();
//println!("{}", tline);
//}
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = [
            "p=0,4 v=3,-3".to_string(),
            "p=6,3 v=-1,-3".to_string(),
            "p=10,3 v=-1,2".to_string(),
            "p=2,0 v=2,-1".to_string(),
            "p=0,0 v=1,3".to_string(),
            "p=3,0 v=-2,-2".to_string(),
            "p=7,6 v=-1,-3".to_string(),
            "p=3,0 v=-1,-2".to_string(),
            "p=9,3 v=2,3".to_string(),
            "p=7,3 v=-1,2".to_string(),
            "p=2,4 v=2,-3".to_string(),
            "p=9,5 v=-3,-3".to_string(),
        ];
        assert_eq!(part_1(&data, 100, 11, 7), 12);
    }
}
