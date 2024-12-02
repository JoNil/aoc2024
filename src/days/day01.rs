use std::collections::HashMap;

pub static INPUT: &str = include_str!("../input/1.txt");
pub static TEST_INPUT: &str = include_str!("../input/1_test.txt");

pub fn a(input: &str) -> i32 {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for line in input.lines() {
        let (first, second) = line.split_once(" ").unwrap();

        a.push(first.trim().parse::<i32>().unwrap());
        b.push(second.trim().parse::<i32>().unwrap());
    }

    a.sort();
    b.sort();

    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 11);
    assert_eq!(a(INPUT), 1938424);
}

pub fn b(input: &str) -> i32 {
    let mut a = Vec::new();
    let mut b = HashMap::new();

    for line in input.lines() {
        let (first, second) = line.split_once(" ").unwrap();

        a.push(first.trim().parse::<i32>().unwrap());
        *b.entry(second.trim().parse::<i32>().unwrap()).or_insert(0) += 1;
    }

    a.iter().map(|a| a * b.get(a).unwrap_or(&0)).sum::<i32>()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 31);
    assert_eq!(b(INPUT), 22014209);
}
