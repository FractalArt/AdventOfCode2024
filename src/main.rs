use aoc2024::read_data;
use std::fs;

fn main() {
    // day 01
    let data_01 = read_data::<String, _>("data/day_01.txt").unwrap();
    println!("[Day 01 - Task 1]: {}", aoc2024::day_01::day_01_1(&data_01));
    println!("[Day 01 - Task 2]: {}", aoc2024::day_01::day_01_2(&data_01));
    // day 02
    let data_02 = read_data::<String, _>("data/day_02.txt").unwrap();
    println!("[Day 02 - Task 1]: {}", aoc2024::day_02::day_02_1(&data_02));
    println!("[Day 02 - Task 2]: {}", aoc2024::day_02::day_02_2(&data_02));
    // day 03
    let data_03 = fs::read_to_string("data/day_03.txt").unwrap();
    println!("[Day 03 - Task 1]: {}", aoc2024::day_03::day_03_1(&data_03));
    println!("[Day 03 - Task 2]: {}", aoc2024::day_03::day_03_2(&data_03));
}
