use std::collections::HashSet;

pub static INPUT: &str = include_str!("../input/5.txt");
pub static TEST_INPUT: &str = include_str!("../input/5_test.txt");

pub fn a(input: &str) -> i32 {
    let (rules_raw, updates_raw) = input.split_once("\n\n").unwrap();

    let mut rules = Vec::<(i32, i32)>::new();

    for rule in rules_raw.lines() {
        let (a, b) = rule.split_once('|').unwrap();
        rules.push((a.parse().unwrap(), b.parse().unwrap()));
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

    'update: for update in &updates {
        let mut has_seen = HashSet::<i32>::new();
        let all_nums = HashSet::<i32>::from_iter(update.iter().copied());
        for num in update {
            for (first, second) in &rules {
                if num == second && all_nums.contains(first) && !has_seen.contains(first) {
                    continue 'update;
                }
            }

            has_seen.insert(*num);
        }

        let middle = update[update.len() / 2];

        sum_of_middle_page_no += middle;
    }

    sum_of_middle_page_no
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 143);
    assert_eq!(a(INPUT), 5639);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
