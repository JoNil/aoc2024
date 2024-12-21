pub static INPUT: &str = include_str!("../input/21.txt");
pub static TEST_INPUT: &str = include_str!("../input/21_test.txt");

fn find_shortest_sequence(code: &[u8]) -> i32 {}

pub fn a(input: &str) -> i32 {
    let codes = input
        .lines()
        .map(|s| {
            (
                s.as_bytes(),
                s.trim_end_matches('A').parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum_of_complexity = 0;

    for (code, code_no) in codes {
        let complexity = find_shortest_sequence(code);
        sum_of_complexity += code_no * complexity;
    }

    sum_of_complexity
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
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
