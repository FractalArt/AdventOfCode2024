//! # Advent of Code 2024 - Day 11
//!
//! This module contains the solution of the [eleventh day's challenges](https://adventofcode.com/2024/day/11).

/// The solution to task 1 of day 11.
pub fn part_1_2(data: &str, blinks: usize) -> usize {
    let mut data: Vec<usize> = data.split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    for _ in 1..=blinks {
       data = data.into_iter()
           .flat_map(|x| {
                let str = format!("{x}");
                match x {
                    0 => vec![1].into_iter(),
                    _ if str.len() % 2 == 0 => {
                        vec![
                            str[0..str.len() / 2].parse::<usize>().unwrap(),
                            str[str.len() / 2..].parse::<usize>().unwrap(),
                        ].into_iter()
                    },
                    _ => vec![2024 * x].into_iter(),
                }
           }).collect();
    }

    data.len()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_2() {
        let data = "125 17";
        assert_eq!(part_1_2(data, 6), 22);
        assert_eq!(part_1_2(data, 25), 55312);
    }

    //#[test]
    //fn test_part_2() {
        //let data = "125 17";
        //assert_eq!(part_2(data), 81);
    //}
}
