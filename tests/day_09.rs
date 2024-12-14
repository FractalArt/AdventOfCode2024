#[test]
fn test_day_09() {
    let data = std::fs::read_to_string("data/day_09.txt").unwrap();
    let data = data.trim();
    let task_1 = aoc2024::day_09::part_1(data);
    assert_eq!(task_1, 6262891638328);
    let task_2 = aoc2024::day_09::part_2(data);
    assert_eq!(task_2, 6287317016845);
}
