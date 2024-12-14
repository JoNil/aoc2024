use glam::IVec2;

pub static INPUT: &str = include_str!("../input/13.txt");
pub static TEST_INPUT: &str = include_str!("../input/13_test.txt");

#[derive(Default)]
struct Machine {
    a: IVec2,
    b: IVec2,
    p: IVec2,
}

pub fn a(input: &str) -> i32 {
    let mut minimum_tokens = 0;

    for machine_str in input.trim().split("\n\n") {
        let mut machine = Machine::default();

        let (line1, rest) = machine_str.split_once('\n').unwrap();
        let (line2, line3) = rest.split_once('\n').unwrap();

        {
            let (left, right) = line1.split_once(',').unwrap();

            machine.a.x = left.trim_start_matches("Button A: X+").parse().unwrap();
            machine.a.y = right.trim_start_matches(" Y+").parse().unwrap();
        }

        {
            let (left, right) = line2.split_once(',').unwrap();

            machine.b.x = left.trim_start_matches("Button B: X+").parse().unwrap();
            machine.b.y = right.trim_start_matches(" Y+").parse().unwrap();
        }

        {
            let (left, right) = line3.split_once(',').unwrap();

            machine.p.x = left.trim_start_matches("Prize: X=").parse().unwrap();
            machine.p.y = right.trim_start_matches(" Y=").parse().unwrap();
        }

        let x = (machine.a.x, machine.b.x, machine.p.x);
        let y = (machine.a.y, machine.b.y, machine.p.y);

        let x2 = (x.0 * y.0, x.1 * y.0, x.2 * y.0);
        let y2 = (y.0 * x.0, y.1 * x.0, y.2 * x.0);

        let x3 = (x2.0 - y2.0, x2.1 - y2.1, x2.2 - y2.2);

        if x3.2 % x3.1 == 0 {
            let b = x3.2 / x3.1;

            let y3 = (y2.0, 0i32, y2.2 - y2.1 * b);

            if y3.2 % y3.0 == 0 {
                let a = y3.2 / y3.0;

                minimum_tokens += a * 3 + b;
            }
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
    let mut minimum_tokens = 0;

    for machine_str in input.trim().split("\n\n") {
        let mut machine = Machine::default();

        let (line1, rest) = machine_str.split_once('\n').unwrap();
        let (line2, line3) = rest.split_once('\n').unwrap();

        {
            let (left, right) = line1.split_once(',').unwrap();

            machine.a.x = left.trim_start_matches("Button A: X+").parse().unwrap();
            machine.a.y = right.trim_start_matches(" Y+").parse().unwrap();
        }

        {
            let (left, right) = line2.split_once(',').unwrap();

            machine.b.x = left.trim_start_matches("Button B: X+").parse().unwrap();
            machine.b.y = right.trim_start_matches(" Y+").parse().unwrap();
        }

        {
            let (left, right) = line3.split_once(',').unwrap();

            machine.p.x = left.trim_start_matches("Prize: X=").parse().unwrap();
            machine.p.y = right.trim_start_matches(" Y=").parse().unwrap();
        }

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
