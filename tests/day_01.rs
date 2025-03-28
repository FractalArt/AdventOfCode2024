use aoc2024::{self, read_data};

#[test]
fn test_day_01() {
    let data = read_data::<String, _>("data/day_01.txt").unwrap();
    let task_1 = aoc2024::day_01::part_1(&data);
    assert_eq!(task_1, 1110981);
    let task_2 = aoc2024::day_01::part_2(&data);
    assert_eq!(task_2, 24869388);
}
