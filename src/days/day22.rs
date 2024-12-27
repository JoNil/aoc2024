use crate::{AdventHashMap, AdventHashSet};

pub static INPUT: &str = include_str!("../input/22.txt");
pub static TEST_INPUT: &str = include_str!("../input/22_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/22_test_2.txt");

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a & (16777216 - 1)
}

pub fn a(input: &str) -> i64 {
    let mut sum_of_secret_numbers = 0;

    for mut value in input.lines().map(|l| l.parse::<i64>().unwrap()) {
        for _ in 0..2000 {
            value = prune(mix(value, value << 6));
            value = prune(mix(value, value >> 5));
            value = prune(mix(value, value << 11));
        }

        sum_of_secret_numbers += value;
    }

    sum_of_secret_numbers
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 37327623);
    assert_eq!(a(INPUT), 14726157693);
}

pub fn b(input: &str) -> i32 {
    let mut price_tables = Vec::new();

    for mut value in input.lines().map(|l| l.parse::<i64>().unwrap()) {
        let mut price_table = AdventHashMap::default();

        let mut last_price = value % 10;
        let mut diffs = [0i8; 4];

        for i in 0..2000 {
            value = prune(mix(value, value << 6));
            value = prune(mix(value, value >> 5));
            value = prune(mix(value, value << 11));

            let price = value % 10;
            let diff = price - last_price;
            last_price = price;
            diffs.rotate_left(1);
            diffs[3] = diff as i8;

            if i > 2 {
                price_table.entry(diffs).or_insert(price as i8);
            }
        }

        price_tables.push(price_table);
    }

    let possible_sequences = price_tables
        .iter()
        .flat_map(|t| t.keys())
        .collect::<AdventHashSet<_>>();

    possible_sequences
        .into_iter()
        .map(|s| {
            price_tables
                .iter()
                .map(|pt| *pt.get(s).unwrap_or(&0) as i32)
                .sum::<i32>()
        })
        .max()
        .unwrap_or(0)
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT_2), 23);
    assert_eq!(b(INPUT), 1614);
}
