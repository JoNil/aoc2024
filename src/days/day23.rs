use crate::{AdventHashMap, AdventHashSet};

pub static INPUT: &str = include_str!("../input/23.txt");
pub static TEST_INPUT: &str = include_str!("../input/23_test.txt");

pub fn a(input: &str) -> i32 {
    let mut computers = AdventHashMap::<&str, AdventHashSet<&str>>::default();

    for (a, b) in input.lines().map(|l| l.split_once('-').unwrap()) {
        computers.entry(a).or_default().insert(b);
        computers.entry(b).or_default().insert(a);
    }

    let mut path_count = 0;

    for (start, next) in computers.iter().filter(|c| c.0.starts_with('t')) {
        for second in next {
            let second_next = computers.get(second).unwrap();
            for third in second_next {
                if next.contains(third) {
                    path_count += 1;
                    println!("{start} {second} {third}");
                }
            }
        }
    }

    path_count
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 7);
    assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
