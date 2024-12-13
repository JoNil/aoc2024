use glam::ivec2;
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
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<Machine>().unwrap())
        .collect::<Vec<_>>();

    let mut minimum_tokens = 0;

    for machine in machines {
        let mut min = None;

        for i in 0..100 {
            for j in 0..100 {
                let p = ivec2(machine.a.x, machine.a.y) * i + ivec2(machine.b.x, machine.b.y) * j;
                if p == ivec2(machine.p.x, machine.p.y) {
                    min = Some(min.unwrap_or(i32::MAX).min(i * 3 + j));
                }
            }
        }

        if let Some(min) = min {
            minimum_tokens += min;
        }
    }

    minimum_tokens
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 480);
    assert_eq!(a(INPUT), 30413);
}

pub fn b(input: &str) -> i64 {
    let machines = input
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<Machine>().unwrap())
        .collect::<Vec<_>>();

    let mut minimum_tokens = 0;

    for machine in machines {
        let x = (
            machine.a.x as i64,
            machine.b.x as i64,
            machine.p.x as i64 + 10000000000000,
        );
        let y = (
            machine.a.y as i64,
            machine.b.y as i64,
            machine.p.y as i64 + 10000000000000,
        );

        let x2 = (x.0 * y.0, x.1 * y.0, x.2 * y.0);
        let y2 = (y.0 * x.0, y.1 * x.0, y.2 * x.0);

        let x3 = (x2.0 - y2.0, x2.1 - y2.1, x2.2 - y2.2);

        if x3.2 % x3.1 == 0 {
            let b = x3.2 / x3.1;

            let y3 = (y2.0, 0i64, y2.2 - y2.1 * b);

            if y3.2 % y3.0 == 0 {
                let a = y3.2 / y3.0;

                minimum_tokens += a * 3 + b;
            }
        }
    }

    minimum_tokens
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 875318608908);
    assert_eq!(b(INPUT), 92827349540204);
}
