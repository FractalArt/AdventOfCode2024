use std::fs;

#[test]
fn test_day_03() {
    let data = fs::read_to_string("data/day_03.txt").unwrap();
    let task_1 = aoc2024::day_03::day_03_1(&data);
    assert_eq!(task_1, 161289189);
    let task_2 = aoc2024::day_03::day_03_2(&data);
    assert_eq!(task_2, 83595109);
}
