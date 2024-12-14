use glam::IVec2;
use std::{mem, str};

pub static INPUT: &str = include_str!("../input/14.txt");
pub static TEST_INPUT: &str = include_str!("../input/14_test.txt");

#[derive(Clone)]
struct Map {
    data: Vec<i8>,
    width: i8,
    height: i8,
}

impl Map {
    fn empty(width: i32, height: i32) -> Map {
        Map {
            data: vec![0; (width * height) as usize],
            width: width as i8,
            height: height as i8,
        }
    }

    fn modify(&mut self, x: i8, y: i8, delta: i8) -> i8 {
        if x < 0 || x >= self.width {
            return 0;
        }

        if y < 0 || y >= self.height {
            return 0;
        }

        let index = x as i32 + y as i32 * self.width as i32;

        let new = self.data[index as usize] + delta;
        self.data[index as usize] = new;

        new
    }
}

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

pub fn b(input: &str, size: IVec2) -> i32 {
    let mut robots = Robots::default();

    let mut map = Map::empty(size.x, size.y);
    let mut map2 = Map::empty(size.x, size.y);

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

        map.modify(x, y, 1);
    }

    let mut step = 0;

    loop {
        step += 1;

        let mut conflict_count = 0;

        for i in 0..robots.pos_x.len() {
            let x = robots.pos_x[i];
            let y = robots.pos_y[i];

            map2.modify(x, y, -1);

            let new_x =
                (robots.pos_x[i] as i16 + robots.speed_x[i] as i16).rem_euclid(size.x as _) as i8;
            let new_y =
                (robots.pos_y[i] as i16 + robots.speed_y[i] as i16).rem_euclid(size.y as _) as i8;

            let robots_in_pos = map.modify(new_x, new_y, 1);

            if robots_in_pos > 1 {
                conflict_count += 1;
            }

            robots.pos_x[i] = new_x;
            robots.pos_y[i] = new_y;
        }

        mem::swap(&mut map, &mut map2);

        if conflict_count == 0 {
            break;
        }
    }

    step
}

#[test]
fn test_b() {
    assert_eq!(b(INPUT, glam::ivec2(101, 103)), 7858);
}
