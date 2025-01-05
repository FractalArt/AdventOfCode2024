use aoc2024::{self, read_data};

#[test]
fn test_day_22() {
    let data = read_data::<String, _>("data/day_22.txt").unwrap();
    let task_1 = aoc2024::day_22::part_1(&data, 2000);
    assert_eq!(task_1, 20215960478);
}
