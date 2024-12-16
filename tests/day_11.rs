#[test]
fn test_day_11() {
    let data = std::fs::read_to_string("data/day_11.txt").unwrap();
    let task_1 = aoc2024::day_11::part_1_2(&data, 25);
    assert_eq!(task_1, 186203);
    //let task_2 = aoc2024::day_11::part_2(&data);
    //assert_eq!(task_2, 1541);
}
