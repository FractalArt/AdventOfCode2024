use aoc2024::read_data;
use colored::Colorize;
use std::fs;

fn main() {
    // day 06
    let data = read_data::<String, _>("data/day_06.txt").unwrap();
    println!(
        "[Day 06 - Task 2]: {}",
        format!("{}", aoc2024::day_06::part_2(&data)).green()
    );
}
