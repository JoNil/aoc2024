use glam::{ivec2, IVec2};
use std::{fmt::Display, str};

pub static INPUT: &str = include_str!("../input/14.txt");
pub static TEST_INPUT: &str = include_str!("../input/14_test.txt");

#[derive(Clone)]
struct Map {
    data: Vec<u8>,
    width: i32,
    height: i32,
}

impl Map {
    fn empty(width: i32, height: i32) -> Map {
        Map {
            data: vec![b'.'; (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, pos: IVec2) -> u8 {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return b'.';
        }

        if pos.y < 0 || pos.y >= self.height {
            return b'.';
        }

        self.data[index as usize]
    }

    fn set(&mut self, pos: IVec2, new: u8) -> bool {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return false;
        }

        if pos.y < 0 || pos.y >= self.height {
            return false;
        }

        self.data[index as usize] = new;

        true
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            writeln!(f, "{}", str::from_utf8(line).unwrap())?;
        }

        Ok(())
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
    assert_eq!(a(TEST_INPUT, ivec2(11, 7)), 12);
    assert_eq!(a(INPUT, ivec2(101, 103)), 221655456);
}

pub fn b(input: &str, size: IVec2) -> i32 {
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

    let mut map = Map::empty(size.x, size.y);

    for i in 0..robots.pos_x.len() {
        let x = robots.pos_x[i];
        let y = robots.pos_y[i];

        map.set(ivec2(x as _, y as _), b'x');
    }

    println!("{}", map);

    0
}

#[test]
fn test_b() {
    assert_eq!(b(INPUT, ivec2(101, 103)), 0);
}
