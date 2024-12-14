use glam::IVec2;
use std::str;

pub static INPUT: &str = include_str!("../input/14.txt");
pub static TEST_INPUT: &str = include_str!("../input/14_test.txt");

#[derive(Default, Debug)]
struct Robots {
    pos_x: Vec<i8>,
    pos_y: Vec<i8>,
    speed_x: Vec<i8>,
    speed_y: Vec<i8>,
}

pub fn a(input: &str, size: IVec2) -> i32 {
    let mut robots = Robots::default();

    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();

        let (x, y) = left.split_once(',').unwrap();
        let x = x.trim_start_matches("p=").parse().unwrap();
        let y = y.parse().unwrap();

        let (dx, dy) = right.split_once(',').unwrap();
        let dx = dx.trim_start_matches("v=").parse().unwrap();
        let dy = dy.parse().unwrap();

        robots.pos_x.push(x);
        robots.pos_y.push(y);
        robots.speed_x.push(dx);
        robots.speed_y.push(dy);
    }

    for _step in 0..100 {
        for i in 0..robots.pos_x.len() {
            robots.pos_x[i] =
                (robots.pos_x[i] as i16 + robots.speed_x[i] as i16).rem_euclid(size.x as _) as i8;
            robots.pos_y[i] =
                (robots.pos_y[i] as i16 + robots.speed_y[i] as i16).rem_euclid(size.y as _) as i8;
        }
    }

    let middle_x = size.x as i8 / 2;
    let middle_y = size.y as i8 / 2;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for i in 0..robots.pos_x.len() {
        let x = robots.pos_x[i];
        let y = robots.pos_y[i];

        if x < middle_x && y < middle_y {
            q1 += 1;
        } else if x > middle_x && y < middle_y {
            q2 += 1;
        } else if x < middle_x && y > middle_y {
            q3 += 1;
        } else if x > middle_x && y > middle_y {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT, glam::ivec2(11, 7)), 12);
    assert_eq!(a(INPUT, glam::ivec2(101, 103)), 221655456);
}
