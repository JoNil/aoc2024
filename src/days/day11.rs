use crate::AdventHashMap;
use std::mem;

pub static INPUT: &str = include_str!("../input/11.txt");
pub static TEST_INPUT: &str = include_str!("../input/11_test.txt");

fn count_digits(mut n: i64) -> i64 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;

    while n != 0 {
        n /= 10;
        count += 1;
    }

    count
}

pub fn a(input: &str) -> i64 {
    let mut numbers = input
        .trim()
        .split(' ')
        .map(|n| (n.parse().unwrap(), 1))
        .collect::<AdventHashMap<i64, i64>>();

    let mut new_numbers = AdventHashMap::default();

    for _round in 0..25 {
        new_numbers.clear();

        for (number, count) in &numbers {
            if *number == 0 {
                *new_numbers.entry(1).or_default() += *count;
            } else {
                let digit_count = count_digits(*number);

                if digit_count % 2 == 0 {
                    let mut left = *number;
                    let mut right = 0;

                    for i in 0..digit_count / 2 {
                        let lower_digit = left % 10;
                        left /= 10;
                        right += lower_digit * 10i64.pow(i as _);
                    }

                    *new_numbers.entry(left).or_default() += *count;
                    *new_numbers.entry(right).or_default() += *count;
                } else {
                    *new_numbers.entry(*number * 2024).or_default() += *count;
                }
            }
        }

        mem::swap(&mut numbers, &mut new_numbers);
    }

    numbers.values().sum()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 55312);
    assert_eq!(a(INPUT), 194482);
}

pub fn b(input: &str) -> i64 {
    let mut numbers = input
        .trim()
        .split(' ')
        .map(|n| (n.parse().unwrap(), 1))
        .collect::<AdventHashMap<i64, i64>>();

    let mut new_numbers = AdventHashMap::default();

    for _round in 0..75 {
        new_numbers.clear();

        for (number, count) in &numbers {
            if *number == 0 {
                *new_numbers.entry(1).or_default() += *count;
            } else {
                let digit_count = count_digits(*number);

                if digit_count % 2 == 0 {
                    let mut left = *number;
                    let mut right = 0;

                    for i in 0..digit_count / 2 {
                        let lower_digit = left % 10;
                        left /= 10;
                        right += lower_digit * 10i64.pow(i as _);
                    }

                    *new_numbers.entry(left).or_default() += *count;
                    *new_numbers.entry(right).or_default() += *count;
                } else {
                    *new_numbers.entry(*number * 2024).or_default() += *count;
                }
            }
        }

        mem::swap(&mut numbers, &mut new_numbers);
    }

    numbers.values().sum()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 65601038650482);
    assert_eq!(b(INPUT), 232454623677743);
}
