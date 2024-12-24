use aoc2024::{self, read_data};

#[test]
fn test_day_18() {
    let data = read_data::<String, _>("data/day_18.txt").unwrap();
    let task_1 = aoc2024::day_18::part_1(&data, 1024, 70);
    assert_eq!(task_1, Some(380));
    let task_2 = aoc2024::day_18::part_2(&data, 70);
    assert_eq!(task_2, (26, 50));
}
