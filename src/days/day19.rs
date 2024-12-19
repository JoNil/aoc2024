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

pub fn b(input: &str) -> i64 {
    let (pattern_str, design_str) = input.trim().split_once("\n\n").unwrap();

    let patterns = pattern_str.split(", ").collect::<Vec<_>>();

    let mut primitive = Vec::with_capacity(patterns.len());
    let mut combined = Vec::with_capacity(patterns.len());

    for pattern in &patterns {
        if match_pattern_except(&patterns, pattern, pattern) {
            combined.push(*pattern);
        } else {
            primitive.push(*pattern);
        }
    }

    println!("{primitive:?}");
    println!("{combined:?}");

    let mut possible_designs = 0;

    for design in design_str.lines() {
        println!("New: {design}");
        let mut new_possible_designs = if match_pattern(&primitive, design) {
            1
        } else {
            0
        };
        for combined in &combined {
            if design.contains(*combined) {
                new_possible_designs *= 2;
            }
        }
        possible_designs += new_possible_designs;
        println!("Ways: {new_possible_designs}");
    }

    possible_designs
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 16);
    //assert_eq!(b(INPUT), 0);
}
