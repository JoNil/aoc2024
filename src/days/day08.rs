pub static INPUT: &str = include_str!("../input/8.txt");
pub static TEST_INPUT: &str = include_str!("../input/8_test.txt");

pub fn a(input: &str) -> i32 {
    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 14);
    assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
