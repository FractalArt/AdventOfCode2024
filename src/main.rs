use aoc2024::read_data;
use colored::Colorize;
use std::fs;
use std::time::Instant;

fn main() {
    let overall_start = Instant::now();

    // day 01
    let data = read_data::<String, _>("data/day_01.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_01::part_1(&data);
    println!(
        "[Day 01 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_01::part_2(&data);
    println!(
        "[Day 01 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 02
    let data = read_data::<String, _>("data/day_02.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_02::part_1(&data);
    println!(
        "[Day 02 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_02::part_2(&data);
    println!(
        "[Day 02 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 03
    let data = fs::read_to_string("data/day_03.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_03::part_1(&data);
    println!(
        "[Day 03 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_03::part_2(&data);
    println!(
        "[Day 03 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 04
    let data = read_data::<String, _>("data/day_04.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_04::part_1(&data);
    println!(
        "[Day 04 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_04::part_2(&data);
    println!(
        "[Day 04 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 05
    let data = read_data::<String, _>("data/day_05.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_05::part_1(&data);
    println!(
        "[Day 05 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_05::part_2(&data);
    println!(
        "[Day 05 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );
    // day 06
    let data = read_data::<String, _>("data/day_06.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_06::part_1(&data);
    println!(
        "[Day 06 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_06::part_2(&data);
    println!(
        "[Day 06 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 07
    let data = read_data::<String, _>("data/day_07.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_07::part_1_2(&data, &['+', '*']);
    println!(
        "[Day 07 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_07::part_1_2(&data, &['+', '*', '|']);
    println!(
        "[Day 07 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 08
    let data = read_data::<String, _>("data/day_08.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_08::part_1(&data);
    println!(
        "[Day 08 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_08::part_2(&data);
    println!(
        "[Day 08 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 09
    let data = std::fs::read_to_string("data/day_09.txt").unwrap();
    let data = data.trim();
    let start = Instant::now();
    let part_1 = aoc2024::day_09::part_1(data);
    println!(
        "[Day 09 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_09::part_2(data);
    println!(
        "[Day 09 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 10
    let data = read_data::<String, _>("data/day_10.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_10::part_1(&data);
    println!(
        "[Day 10 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_10::part_2(&data);
    println!(
        "[Day 10 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 11
    let data = std::fs::read_to_string("data/day_11.txt").unwrap();
    //let data = "125 17";
    let start = Instant::now();
    let part_1 = aoc2024::day_11::part_1_2(&data, 25);
    println!(
        "[Day 11 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    //let start = Instant::now();
    //let part_2 = aoc2024::day_11::part_1_2(&data, 75);
    //println!(
        //"[Day 11 - Task 2][{:>15}]: {}",
        //format!("{:?}", start.elapsed()).blue(),
        //format!("{}", part_2).green()
    //);

    println!(
        "\nTotal time taken: [{}]",
        format!("{:?}", overall_start.elapsed()).blue()
    );
}
