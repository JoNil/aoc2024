use fxhash::FxHashMap;

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

fn match_pattern_b(patterns: &FxHashMap<u8, Vec<&str>>, design: &str) -> i32 {
    if design.is_empty() {
        return 1;
    }

    let mut count = 0;

    if let Some(possible_patterns) = patterns.get(&design.as_bytes()[0]) {
        for pattern in possible_patterns {
            let pattern_len = pattern.len();
            let design_len = design.len();

            if pattern_len > design_len {
                continue;
            }

            if &design[..pattern_len] == *pattern {
                count += match_pattern_b(patterns, &design[pattern_len..]);
            }
        }
    }

    count
}

pub fn b(input: &str) -> i32 {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();

    let patterns = pattern_str.split(", ").collect::<Vec<_>>();
    let mut sorted_patterns: FxHashMap<u8, Vec<&str>> = FxHashMap::default();

    for pattern in patterns {
        let key = pattern.as_bytes()[0];
        sorted_patterns.entry(key).or_default().push(pattern);
    }

    let mut possible_designs = 0;

    for design in design_str.lines() {
        possible_designs += match_pattern_b(&sorted_patterns, design);
    }

    possible_designs
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 16);
    //assert_eq!(b(INPUT), 0);
}
