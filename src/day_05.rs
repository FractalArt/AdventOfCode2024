//! # Advent of Code 2024 - Day 5
//!
//! This module contains the solution of the [fifth day's challenges](https://adventofcode.com/2024/day/5).
use std::collections::HashMap as HM;

fn parse_input(data: &[String]) -> (HM<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let rules = data
        .iter()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let mut split = s.split('|');
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold(HM::<usize, Vec<_>>::new(), |mut acc, (key, val)| {
            acc.entry(key).or_default().push(val);
            acc
        });

    let page_numbers = data
        .iter()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|s| s.split(",").map(|i| i.parse::<usize>().unwrap()).collect())
        .collect();

    (rules, page_numbers)
}

/// The solution to task 1 of day 5.
pub fn day_05_1(data: &[String]) -> usize {
    let (rules, page_numbers) = parse_input(data);
    page_numbers
        .iter()
        .filter(|pn| {
            pn.iter().enumerate().all(|(i, n)| {
                !pn[..i]
                    .iter()
                    .any(|b| rules.get(n).unwrap_or(&vec![]).contains(b))
            })
        })
        .map(|pn| pn[pn.len() / 2])
        .sum()
}

///// The solution to task 2 of day 5.
//pub fn day_05_2(data: &[String]) -> usize {
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_05_1() {
        let data = [
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
            "".to_string(),
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ];
        assert_eq!(day_05_1(&data), 143);
    }

    //#[test]
    //fn test_day_05_2() {
    //let data = [
    //"47|53".to_string(),
    //"97|13".to_string(),
    //"97|61".to_string(),
    //"97|47".to_string(),
    //"75|29".to_string(),
    //"61|13".to_string(),
    //"75|53".to_string(),
    //"29|13".to_string(),
    //"97|29".to_string(),
    //"53|29".to_string(),
    //"61|53".to_string(),
    //"97|53".to_string(),
    //"61|29".to_string(),
    //"47|13".to_string(),
    //"75|47".to_string(),
    //"97|75".to_string(),
    //"47|61".to_string(),
    //"75|61".to_string(),
    //"47|29".to_string(),
    //"75|13".to_string(),
    //"53|13".to_string(),
    //"".to_string(),
    //"75,47,61,53,29".to_string(),
    //"97,61,53,29,13".to_string(),
    //"75,29,13".to_string(),
    //"75,97,47,61,53".to_string(),
    //"61,13,29".to_string(),
    //"97,13,75,29,47".to_string(),
    //];
    //assert_eq!(day_05_2(&data), 9);
    //}
}
