use std::{cmp::Ordering, collections::HashSet};

pub static INPUT: &str = include_str!("../input/5.txt");
pub static TEST_INPUT: &str = include_str!("../input/5_test.txt");

pub fn a(input: &str) -> i32 {
    let (rules_raw, updates_raw) = input.split_once("\n\n").unwrap();

    let mut rules = HashSet::<(i32, i32)>::new();

    for rule in rules_raw.lines() {
        let (a, b) = rule.split_once('|').unwrap();
        rules.insert((a.parse().unwrap(), b.parse().unwrap()));
    }

    let mut updates = Vec::new();

    for update in updates_raw.lines() {
        updates.push(
            update
                .split(',')
                .map(|u| u.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut sum_of_middle_page_no = 0;

    for update in &updates {
        let mut sorted_update = update.clone();
        sorted_update.sort_unstable_by(|a, b| {
            if rules.contains(&(*a, *b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        if *update == sorted_update {
            let middle = update[update.len() / 2];
            sum_of_middle_page_no += middle;
        }
    }

    sum_of_middle_page_no
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 143);
    assert_eq!(a(INPUT), 5639);
}

pub fn b(input: &str) -> i32 {
    let (rules_raw, updates_raw) = input.split_once("\n\n").unwrap();

    let mut rules = HashSet::<(i32, i32)>::new();

    for rule in rules_raw.lines() {
        let (a, b) = rule.split_once('|').unwrap();
        rules.insert((a.parse().unwrap(), b.parse().unwrap()));
    }

    let mut updates = Vec::new();

    for update in updates_raw.lines() {
        updates.push(
            update
                .split(',')
                .map(|u| u.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut sum_of_middle_page_no = 0;

    for update in &updates {
        let mut sorted_update = update.clone();
        sorted_update.sort_unstable_by(|a, b| {
            if rules.contains(&(*a, *b)) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        if *update != sorted_update {
            let middle = sorted_update[sorted_update.len() / 2];
            sum_of_middle_page_no += middle;
        }
    }

    sum_of_middle_page_no
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 123);
    assert_eq!(b(INPUT), 5273);
}
