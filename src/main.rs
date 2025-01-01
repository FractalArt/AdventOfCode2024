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
    let start = Instant::now();
    let part_2 = aoc2024::day_11::part_1_2(&data, 75);
    println!(
        "[Day 11 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2).green()
    );

    // day 14
    let data = read_data::<String, _>("data/day_14.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_14::part_1(&data, 100, 101, 103);
    println!(
        "[Day 14 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_14::part_2(&data, 101, 103);
    println!(
        "[Day 14 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_2.unwrap()).green()
    );

    // day 17
    let start = Instant::now();
    let part_1 = aoc2024::day_17::part_1(
        vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 3, 5, 5, 3, 0],
        34615120,
        0,
        0,
    );
    println!(
        "[Day 17 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        part_1.green()
    );
    let start = Instant::now();
    let reversed = |a: usize| (a % 8) ^ 3 ^ (a / 2usize.pow(((a % 8) ^ 5) as u32) % 8);
    let part_2 = aoc2024::day_17::part_2(
        vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 3, 5, 5, 3, 0],
        reversed,
    );
    println!(
        "[Day 17 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{:?}", part_2).green()
    );

    // day 18
    let data = read_data::<String, _>("data/day_18.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_18::part_1(&data, 1024, 70);
    println!(
        "[Day 18 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1.unwrap()).green()
    );
    let start = Instant::now();
    let part_2 = aoc2024::day_18::part_2(&data, 70);
    println!(
        "[Day 18 - Task 2][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("({}, {})", part_2.0, part_2.1).green()
    );

    // day 24
    let data = std::fs::read_to_string("data/day_24.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_24::part_1(&data);
    println!(
        "[Day 24 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );

    // day 25
    let data = std::fs::read_to_string("data/day_25.txt").unwrap();
    let start = Instant::now();
    let part_1 = aoc2024::day_25::part_1(&data);
    println!(
        "[Day 25 - Task 1][{:>15}]: {}",
        format!("{:?}", start.elapsed()).blue(),
        format!("{}", part_1).green()
    );

    // Overall
    println!(
        "\nTotal time taken: [{}]",
        format!("{:?}", overall_start.elapsed()).blue()
    );
}
