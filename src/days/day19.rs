use crate::AdventHashMap;

pub static INPUT: &str = include_str!("../input/19.txt");
pub static TEST_INPUT: &str = include_str!("../input/19_test.txt");

fn match_pattern(patterns: &[&str], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();

        if pattern_len > design_len {
            continue;
        }

        if &design[..pattern_len] == *pattern && match_pattern(patterns, &design[pattern_len..]) {
            return true;
        }
    }

    false
}

pub fn a(input: &str) -> i32 {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();

    let mut patterns = pattern_str.split(", ").collect::<Vec<_>>();
    patterns.sort_by_key(|a| a.len());

    let mut possible_designs = 0;

    for design in design_str.lines() {
        if match_pattern(&patterns, design) {
            possible_designs += 1;
        }
    }

    possible_designs
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 6);
    assert_eq!(a(INPUT), 363);
}

fn count_patterns<'a>(
    patterns: &[&str],
    design: &'a str,
    cache: &mut AdventHashMap<&'a str, i64>,
) -> i64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = cache.get(design) {
        return *count;
    }

    let mut count = 0;

    for pattern in patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();

        if pattern_len > design_len {
            continue;
        }

        if &design[..pattern_len] == *pattern {
            count += count_patterns(patterns, &design[pattern_len..], cache);
        }
    }

    cache.insert(design, count);

    count
}

pub fn b(input: &str) -> i64 {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();

    let mut patterns = pattern_str.split(", ").collect::<Vec<_>>();
    patterns.sort_by_key(|a| a.len());

    let designs = design_str.lines().collect::<Vec<_>>();

    designs
        .iter()
        .map(|design| {
            let mut cache = AdventHashMap::default();
            count_patterns(&patterns, design, &mut cache)
        })
        .sum()
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 16);
    assert_eq!(b(INPUT), 642535800868438);
}
