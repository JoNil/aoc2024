use std::collections::HashMap;

use glam::{ivec2, IVec2};
use pathfinding::prelude::dijkstra_all;

pub static INPUT: &str = include_str!("../input/21.txt");
pub static TEST_INPUT: &str = include_str!("../input/21_test.txt");

fn pos_from_digit(digit: u8) -> IVec2 {
    match digit {
        b'7' => ivec2(0, 0),
        b'8' => ivec2(1, 0),
        b'9' => ivec2(2, 0),

        b'4' => ivec2(0, 1),
        b'5' => ivec2(1, 1),
        b'6' => ivec2(2, 1),

        b'1' => ivec2(0, 2),
        b'2' => ivec2(1, 2),
        b'3' => ivec2(2, 2),

        b'0' => ivec2(1, 3),
        b'A' => ivec2(2, 3),
        _ => panic!("Invalid"),
    }
}

fn pos_from_dir(dir: IVec2) -> IVec2 {
    match dir {
        IVec2 { x: 1, y: 0 } => ivec2(2, 1),
        IVec2 { x: -1, y: 0 } => ivec2(0, 1),
        IVec2 { x: 0, y: 1 } => ivec2(1, 0),
        IVec2 { x: 0, y: -1 } => ivec2(1, 1),

        _ => panic!("Invalid"),
    }
}

fn path_keypad(start: IVec2, end: IVec2) {
    let paths: HashMap<IVec2, (IVec2, i32)> = dijkstra_all(&start, |pos| {
        [
            (pos + ivec2(1, 0), 1),
            (pos + ivec2(-1, 0), 1),
            (pos + ivec2(0, 1), 1),
            (pos + ivec2(0, -1), 1),
        ]
        .into_iter()
        .filter(|d| (0..3).contains(&d.0.x) && (0..2).contains(&d.0.y))
    });

    println!("{:#?}", paths);
}

fn find_shortest_sequence(code: &[u8]) -> i32 {
    let mut start = b'A';
    for &end in code {
        println!("{} -> {}", start as char, end as char);
        path_keypad(pos_from_digit(start), pos_from_digit(end));
        start = end;
    }
    0
}

pub fn a(input: &str) -> i32 {
    let codes = input
        .lines()
        .map(|s| {
            (
                s.as_bytes(),
                s.trim_end_matches('A').parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum_of_complexity = 0;

    for (code, code_no) in codes {
        let complexity = find_shortest_sequence(code);
        sum_of_complexity += code_no * complexity;
    }

    sum_of_complexity
}

#[test]
fn test_a() {
    assert_eq!(a("029A"), 1);
    assert_eq!(a(TEST_INPUT), 126384);
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
