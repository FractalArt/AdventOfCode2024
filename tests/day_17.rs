#[test]
fn test_day_17() {
    let task_1 = aoc2024::day_17::part_1(
        vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 3, 5, 5, 3, 0],
        34615120,
        0,
        0,
    );
    assert_eq!(task_1, "1,2,3,1,3,2,5,3,1".to_string());
    let reversed = |a: usize| (a % 8) ^ 3 ^ (a / 2usize.pow(((a % 8) ^ 5) as u32) % 8);
    let task_2 = aoc2024::day_17::part_2(
        vec![2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 3, 5, 5, 3, 0],
        reversed,
    );
    assert_eq!(task_2, 105706277661082);
}
