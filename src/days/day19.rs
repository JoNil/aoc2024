use std::cmp::Ordering;

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

fn match_pattern_pre_b(patterns: &[&str], design: &str) -> i32 {
    if design.is_empty() {
        return 1;
    }

    let mut count = 0;

    for pattern in patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();

        if pattern_len > design_len {
            continue;
        }

        if &design[..pattern_len] == *pattern {
            count += match_pattern_pre_b(patterns, &design[pattern_len..]);
        }
    }

    count
}

fn match_pattern_b(patterns: &[(String, i64)], design: &str) -> i64 {
    if design.is_empty() {
        return 1;
    }

    //println!("D: {design}");

    for (pattern, possible) in patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();

        if pattern_len > design_len {
            continue;
        }

        if design[..pattern_len] == *pattern {
            //println!("P: {pattern} {possible}");
            return possible * match_pattern_b(patterns, &design[pattern_len..]);
        }
    }

    0
}

pub fn b(input: &str) -> i64 {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();

    let patterns = pattern_str.split(", ").collect::<Vec<_>>();
    let mut patterns_with_ways = Vec::with_capacity(patterns.len());

    for pattern in &patterns {
        let possible_ways = match_pattern_pre_b(&patterns, pattern) as i64;
        patterns_with_ways.push((pattern.to_string(), possible_ways));
    }

    for (i, pattern) in patterns.iter().enumerate() {
        for other in &patterns[(i + 1)..] {
            let combined = format!("{pattern}{other}");
            if !patterns.contains(&combined.as_str()) {
                let possible_ways = match_pattern_pre_b(&patterns, &combined) as i64;
                patterns_with_ways.push((combined, possible_ways));
            }
        }
    }

    patterns_with_ways.sort_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => b.0.len().cmp(&a.0.len()),
        Ordering::Greater => Ordering::Greater,
    });

    let mut possible_designs = 0;

    for design in design_str.lines() {
        println!("New: {design}");
        let new_possible_designs = match_pattern_b(&patterns_with_ways, design);
        possible_designs += new_possible_designs;
        println!("Ways: {new_possible_designs}");
    }

    possible_designs
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 16);
    assert_eq!(b(INPUT), 0);
}
