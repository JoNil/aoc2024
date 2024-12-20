use aho_corasick::AhoCorasick;

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

pub fn b(input: &str) -> i64 {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();

    let patterns = pattern_str
        .split(", ")
        .map(|s| s.as_bytes())
        .collect::<Vec<_>>();

    let mut possible_patterns = 0;

    let ac = AhoCorasick::new(patterns).unwrap();

    let mut hits = Vec::new();

    for design in design_str.lines().map(|s| s.as_bytes()) {
        hits.fill(0);
        hits.resize(design.len() + 1, 0);

        hits[0] = 1;

        for hit in ac.find_overlapping_iter(design) {
            hits[hit.end()] += hits[hit.start()];
        }

        possible_patterns += hits[design.len()];
    }

    possible_patterns
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 16);
    assert_eq!(b(INPUT), 642535800868438);
}
