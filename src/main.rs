use aoc2024::read_data;

fn main() {
    // day 01
    let data_01 = read_data::<String, _>("data/day_01.txt").unwrap();
    println!("[Day 01 - Task 1]: {}", aoc2024::day_01::day_01_1(&data_01));
    println!("[Day 01 - Task 2]: {}", aoc2024::day_01::day_01_2(&data_01));

}
