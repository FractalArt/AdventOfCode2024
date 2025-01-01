#[test]
fn test_day_25() {
    let data = std::fs::read_to_string("data/day_25.txt").unwrap();
    let task_1 = aoc2024::day_25::part_1(&data);
    assert_eq!(task_1, 3264);
}
