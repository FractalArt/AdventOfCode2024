use aoc2024::read_data;
use colored::Colorize;

fn main() {
    // day 07
    let task = std::env::args().skip(1).next().unwrap_or("1".to_string()).parse::<u32>().expect("Command-line argument needs to be 1 or 2");
    let operators = if task == 1 {
        vec!['+', '*']
    } else if task == 2 {
        vec!['+', '*', '|']
    } else {
        eprintln!("Command-line argument needs to be 1 or 2");
        std::process::exit(1);
    };

    let data = read_data::<String, _>("data/day_07.txt").unwrap();
    println!(
        "[Day 07 - Task {}]: {}", 
        format!("{}", task).green(),
        format!("{}", aoc2024::day_07::part_1_2(&data, &operators)).green()
    );
}
