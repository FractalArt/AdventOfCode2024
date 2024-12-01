use aoc2024::{self, read_data};

#[test]
fn test_day_01() {
    let data = read_data::<String, _>("data/day_01.txt").unwrap();
    let task_1 = aoc2024::day_01::day_01_1(&data);
    assert_eq!(task_1, 1110981);
}
