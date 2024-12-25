//! # Advent of Code 2024 - Day 14
//!
//! This module contains the solution of the [fourteenth day's challenges](https://adventofcode.com/2024/day/14).


/// The solution to task 1 of day 14.
pub fn part_1(data: &[String], iter: usize, xdim: isize, ydim: isize) -> usize {
    data.iter()
        .map(|line| line.split_whitespace())
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .map(|(pos, vel)| (pos[2..].split(','), vel[2..].split(',')))
        .map(|(mut p_split, mut v_split)| (
                (p_split.next().unwrap().parse::<isize>().unwrap(),
                 p_split.next().unwrap().parse::<isize>().unwrap(),
                 ),
                 (
                     v_split.next().unwrap().parse::<isize>().unwrap(),
                     v_split.next().unwrap().parse::<isize>().unwrap(),
                 )
                                  ))
        .map(|((x, y), (vx, vy))| {
            let mut pos = (x, y);
            for _ in 0..iter {
                pos = ((pos.0 + vx).rem_euclid(xdim), (pos.1 + vy).rem_euclid(ydim));
            }
            pos
        })
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

///// The solution to task 2 of day 14.
//pub fn part_2(data: &[String], xdim: isize, ydim: isize) {
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
