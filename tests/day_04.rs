use aoc2024::{self, read_data};

#[test]
fn test_day_04() {
    let data = read_data::<String, _>("data/day_04.txt").unwrap();
    let task_1 = aoc2024::day_04::day_04_1(&data);
    assert_eq!(task_1, 2500);
    //let task_2 = aoc2024::day_04::day_04_2(&data);
    //assert_eq!(task_2, 296);
}
