use fxhash::{FxHashMap, FxHashSet};

pub static INPUT: &str = include_str!("../input/19.txt");
pub static TEST_INPUT: &str = include_str!("../input/19_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/19_test_2.txt");

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

fn match_pattern_except(patterns: &[&str], design: &str, except: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();

        if pattern_len > design_len {
            continue;
        }

        if *pattern == except {
            continue;
        }

        if &design[..pattern_len] == *pattern
            && match_pattern_except(patterns, &design[pattern_len..], except)
        {
            return true;
        }
    }

    false
}

fn match_primitive<'a>(patterns: &[&'a str], design: &str) -> Option<Vec<&'a str>> {
    if design.is_empty() {
        return Some(Vec::new());
    }

    for pattern in patterns {
        let pattern_len = pattern.len();
        let design_len = design.len();

        if pattern_len > design_len {
            continue;
        }

        if &design[..pattern_len] == *pattern {
            if let Some(mut result) = match_primitive(patterns, &design[pattern_len..]) {
                result.insert(0, pattern);
                return Some(result);
            }
        }
    }

    None
}

fn get_combination<'a>(
    cc: &mut FxHashMap<(&'a str, &'a str), &'a str>,
    combined: &FxHashSet<&'a str>,
    left: &'a str,
    right: &'a str,
) -> Option<&'a str> {
    if let Some(hit) = cc.get(&(left, right)) {
        return Some(hit);
    }

    let combination = format!("{left}{right}");

    if let Some(combined) = combined.get(combination.as_str()) {
        cc.insert((left, right), *combined);
        return Some(combined);
    }

    None
}

fn count_combinations<'a>(
    cc: &mut FxHashMap<(&'a str, &'a str), &'a str>,
    combined: &FxHashSet<&'a str>,
    parts: &[&'a str],
) -> i64 {
    let mut combinations = 1;

    println!("{parts:?}");

    for (i, j) in (0..parts.len() - 1).zip(1..parts.len()) {
        let left = parts[i];
        let right = parts[j];

        if let Some(combination) = get_combination(cc, combined, left, right) {
            let mut new = Vec::with_capacity(parts.len());
            if i != 0 {
                new.extend_from_slice(&parts[..i]);
            }
            new.push(combination);
            if j + 1 < parts.len() {
                new.extend_from_slice(&parts[(j + 1)..]);
            }

            combinations += count_combinations(cc, combined, new.as_slice())
        }
    }

    combinations
}

pub fn b(input: &str) -> i64 {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();

    let patterns = pattern_str.split(", ").collect::<Vec<_>>();

    let mut primitive = Vec::with_capacity(patterns.len());
    let mut combined = FxHashSet::default();
    let mut cc = FxHashMap::<(&str, &str), &str>::default();

    for pattern in &patterns {
        if match_pattern_except(&patterns, pattern, pattern) {
            combined.insert(*pattern);
        } else {
            primitive.push(*pattern);
        }
    }

    println!("{primitive:?}");
    println!("{combined:?}");
    println!();

    let mut possible_designs = 0;

    for design in design_str.lines() {
        if let Some(prim_parts) = match_primitive(&primitive, design) {
            let count = 1 + count_combinations(&mut cc, &combined, prim_parts.as_slice()) - 1;

            possible_designs += count;

            println!("{design} => {count}");
            println!();
        }
    }

    possible_designs
}

#[test]
fn test_b() {
    //assert_eq!(b(TEST_INPUT_2), 6);
    assert_eq!(b(TEST_INPUT), 16);
    //assert_eq!(b(INPUT), 0);
}
