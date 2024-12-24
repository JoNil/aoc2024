pub static INPUT: &str = include_str!("../input/22.txt");
pub static TEST_INPUT: &str = include_str!("../input/22_test.txt");

fn mix(a: i64, b:i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

pub fn a(input: &str) -> i64 {
    
    let mut sum_of_secret_numbers = 0;

    for mut value in input.lines().map(|l|l.parse::<i64>().unwrap()) {

        for _ in 0..2000 {
            value = prune(mix(value, value * 64));
            value = prune(mix(value, value / 32));
            value = prune(mix(value, value * 2048));
        }

        
        sum_of_secret_numbers += value;

    }

    sum_of_secret_numbers

}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 37327623);
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
