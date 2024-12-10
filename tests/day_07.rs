use aoc2024::{self, read_data};

#[test]
fn test_day_07() {
    let data = read_data::<String, _>("data/day_07.txt").unwrap();
    let task_1 = aoc2024::day_07::part_1_2(&data, &['+', '*']);
    assert_eq!(task_1, 4998764814652);
    let task_2 = aoc2024::day_07::part_1_2(&data, &['+', '*', '|']);
    assert_eq!(task_2, 37598910447546);
}
