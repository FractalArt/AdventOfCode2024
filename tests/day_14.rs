use aoc2024::{self, read_data};

#[test]
fn test_day_14() {
    let data = read_data::<String, _>("data/day_14.txt").unwrap();
    let task_1 = aoc2024::day_14::part_1(&data, 100, 101, 103);
    assert_eq!(task_1, 225943500);
    let task_2 = aoc2024::day_14::part_2(&data, 101, 103);
    assert_eq!(task_2, Some(6377));
}
