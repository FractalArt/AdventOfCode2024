//! # Advent of Code 2024 - Day 11
//!
//! This module contains the solution of the [eleventh day's challenges](https://adventofcode.com/2024/day/11).
use itertools::Itertools;

/// The solution to task 1 of day 11.
pub fn part_1_2(data: &str, blinks: usize) -> usize {
    let data: Vec<usize> = data
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    data.into_iter()
        // for each point, step through all the blinks and compute the total number of elements
        .map(|x| {
            println!("### X: {x}");
            let mut counts = vec![x].into_iter().counts();

            for _blink in 1..=blinks {
                println!("--- blink: {_blink}");
                counts = counts.into_iter()
                    .flat_map(|(xx, count)| {
                        let str = format!("{xx}");
                        match xx {
                            0 => vec![1].into_iter().cycle().take(count),
                            _ if str.len() % 2 == 0 => vec![
                                str[0..str.len() / 2].parse::<usize>().unwrap(),
                                str[str.len() / 2..].parse::<usize>().unwrap(),
                            ]
                            .into_iter()
                            .cycle()
                            .take(count * 2),
                            _ => vec![2024 * xx].into_iter().cycle().take(count),
                    }
                    }).counts();
            }
                counts.values().sum::<usize>()

        })
        .sum()

    //for blink in 1..=blinks {
        //println!("blink: {blink}");
        //counts = counts
            //.into_iter()
            //.flat_map(|(x, count)| {
                //let str = format!("{x}");
                //match x {
                    //0 => vec![1].into_iter().cycle().take(count),
                    //_ if str.len() % 2 == 0 => vec![
                        //str[0..str.len() / 2].parse::<usize>().unwrap(),
                        //str[str.len() / 2..].parse::<usize>().unwrap(),
                    //]
                    //.into_iter()
                    //.cycle()
                    //.take(count * 2),
                    //_ => vec![2024 * x].into_iter().cycle().take(count),
                //}
            //})
            //.counts();
    //}

    //counts.values().sum()
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
