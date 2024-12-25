pub static INPUT: &str = include_str!("../input/24.txt");
pub static TEST_INPUT: &str = include_str!("../input/24_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/24_test_2.txt");

pub fn a(input: &str) -> i32 {
    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 0);
    assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 4);
    assert_eq!(b(TEST_INPUT_2), 2024);
    assert_eq!(b(INPUT), 0);
}
