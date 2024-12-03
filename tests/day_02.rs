use aoc2024::{self, read_data};

#[test]
fn test_day_02() {
    let data = read_data::<String, _>("data/day_02.txt").unwrap();
    let task_1 = aoc2024::day_02::day_02_1(&data);
    assert_eq!(task_1, 220);
    //let task_2 = aoc2024::day_02::day_02_2(&data);
    //assert_eq!(task_2, 24869388);
}
