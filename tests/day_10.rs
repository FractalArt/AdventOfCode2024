use aoc2024::{self, read_data};

#[test]
fn test_day_10() {
    let data = read_data::<String, _>("data/day_10.txt").unwrap();
    let task_1 = aoc2024::day_10::part_1(&data);
    assert_eq!(task_1, 746);
    //let task_2 = aoc2024::day_10::part_2(&data);
    //assert_eq!(task_2, 905);
}
