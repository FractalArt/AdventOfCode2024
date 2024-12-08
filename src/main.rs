use aoc2024::read_data;
use colored::Colorize;
use std::fs;

fn main() {
    // day 01
    let data = read_data::<String, _>("data/day_01.txt").unwrap();
    println!(
        "[Day 01 - Task 1]: {}",
        format!("{}", aoc2024::day_01::part_1(&data)).green()
    );
    println!(
        "[Day 01 - Task 2]: {}",
        format!("{}", aoc2024::day_01::part_2(&data)).green()
    );
    // day 02
    let data = read_data::<String, _>("data/day_02.txt").unwrap();
    println!(
        "[Day 02 - Task 1]: {}",
        format!("{}", aoc2024::day_02::part_1(&data)).green()
    );
    println!(
        "[Day 02 - Task 2]: {}",
        format!("{}", aoc2024::day_02::part_2(&data)).green()
    );
    // day 03
    let data = fs::read_to_string("data/day_03.txt").unwrap();
    println!(
        "[Day 03 - Task 1]: {}",
        format!("{}", aoc2024::day_03::part_1(&data)).green()
    );
    println!(
        "[Day 03 - Task 2]: {}",
        format!("{}", aoc2024::day_03::part_2(&data)).green()
    );
    // day 04
    let data = read_data::<String, _>("data/day_04.txt").unwrap();
    println!(
        "[Day 04 - Task 1]: {}",
        format!("{}", aoc2024::day_04::part_1(&data)).green()
    );
    println!(
        "[Day 04 - Task 2]: {}",
        format!("{}", aoc2024::day_04::part_2(&data)).green()
    );
    // day 05
    let data = read_data::<String, _>("data/day_05.txt").unwrap();
    println!(
        "[Day 05 - Task 1]: {}",
        format!("{}", aoc2024::day_05::part_1(&data)).green()
    );
    println!(
        "[Day 05 - Task 2]: {}",
        format!("{}", aoc2024::day_05::part_2(&data)).green()
    );
    // day 06
    let data = read_data::<String, _>("data/day_06.txt").unwrap();
    println!(
        "[Day 06 - Task 1]: {}",
        format!("{}", aoc2024::day_06::part_1(&data)).green()
    );
    println!(
    "[Day 06 - Task 2]: {}",
    format!("{}", aoc2024::day_06::part_2(&data)).green()
    );
}
