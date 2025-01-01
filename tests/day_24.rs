#[test]
fn test_day_24() {
    let data = std::fs::read_to_string("data/day_24.txt").unwrap();
    let task_1 = aoc2024::day_24::part_1(&data);
    assert_eq!(task_1, 47666458872582);
}
