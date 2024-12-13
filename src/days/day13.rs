use parse_display::{Display, FromStr};

pub static INPUT: &str = include_str!("../input/13.txt");
pub static TEST_INPUT: &str = include_str!("../input/13_test.txt");

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Button A: X+{x}, Y+{y}")]
struct ButtonA {
    x: i32,
    y: i32,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Button B: X+{x}, Y+{y}")]
struct ButtonB {
    x: i32,
    y: i32,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Prize: X={x}, Y={y}")]
struct Prize {
    x: i32,
    y: i32,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{a}\n{b}\n{p}")]
struct Machine {
    a: ButtonA,
    b: ButtonB,
    p: Prize,
}

pub fn a(input: &str) -> i32 {
    let machines = input
        .split("\n\n")
        .map(|s| s.parse::<Machine>().unwrap())
        .collect::<Vec<_>>();

    for machine in machines {
        println!("{}", machine);
    }

    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 480);
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
