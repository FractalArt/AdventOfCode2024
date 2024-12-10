#[cfg(not(debug_assertions))]
use aoc2024::{self, read_data};

#[cfg(not(debug_assertions))]
#[test]
fn test_day_06() {
    let data = read_data::<String, _>("data/day_06.txt").unwrap();
    let task_1 = aoc2024::day_06::part_1(&data);
    assert_eq!(task_1, 5129);
    let task_2 = aoc2024::day_06::part_2(&data);
    assert_eq!(task_2, 1888);
}
