use std::{fmt::Display, str};

pub static INPUT: &str = include_str!("../input/6.txt");
pub static TEST_INPUT: &str = include_str!("../input/6_test.txt");

#[derive(Clone)]
struct Map {
    data: Vec<u8>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(input: &str) -> Map {
        let data = input.replace('\n', "").into_bytes();

        let mut width: i32 = 0;
        let mut height: i32 = 0;

        for line in input.lines() {
            width = line.len() as i32;
            height += 1;
        }

        Map {
            data,
            width,
            height,
        }
    }

    fn empty(width: i32, height: i32) -> Map {
        Map {
            data: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        let index = x + y * self.width;

        if x < 0 || x >= self.width {
            return 0;
        }

        if y < 0 || y >= self.height {
            return 0;
        }

        self.data[index as usize]
    }

    fn set(&mut self, x: i32, y: i32, new: u8) {
        let index = x + y * self.width;

        if x < 0 || x >= self.width {
            return;
        }

        if y < 0 || y >= self.height {
            return;
        }

        self.data[index as usize] = new;
    }

    fn set_or(&mut self, x: i32, y: i32, new: u8) {
        let index = x + y * self.width;

        if x < 0 || x >= self.width {
            return;
        }

        if y < 0 || y >= self.height {
            return;
        }

        self.data[index as usize] |= new;
    }

    fn find_first(&self, needle: u8) -> Option<(i32, i32)> {
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.get(x, y);

                if v == needle {
                    return Some((x, y));
                }
            }
        }

        None
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

enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn offset(&self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }

    fn turn_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn bits(&self) -> u8 {
        match self {
            Dir::Up => 0b0001,
            Dir::Right => 0b0010,
            Dir::Down => 0b0100,
            Dir::Left => 0b1000,
        }
    }
}

pub fn a(input: &str) -> i32 {
    let mut map = Map::new(input);
    let mut visited_map = Map::empty(map.width, map.height);

    let mut dir = Dir::Up;
    let mut pos = map.find_first(b'^').unwrap();

    map.set(pos.0, pos.1, b'.');
    visited_map.set(pos.0, pos.1, b'X');

    while pos.0 > 0 && pos.0 < map.width && pos.1 > 0 && pos.1 < map.height {
        let offset = dir.offset();
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

        if map.get(new_pos.0, new_pos.1) == b'#' {
            dir = dir.turn_right();
        } else {
            pos = new_pos;
        }

        visited_map.set(pos.0, pos.1, b'X');
    }

    visited_map.data.iter().filter(|v| **v == b'X').count() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 41);
    assert_eq!(a(INPUT), 4988);
}

pub fn b(input: &str) -> i32 {
    let (map, start_pos) = {
        let mut map = Map::new(input);

        let start_pos = map.find_first(b'^').unwrap();

        map.set(start_pos.0, start_pos.1, b'.');

        (map, start_pos)
    };

    let mut visited_map = Map::empty(map.width, map.height);

    {
        let mut dir = Dir::Up;
        let mut pos = start_pos;

        visited_map.set(pos.0, pos.1, b'X');

        while pos.0 > 0 && pos.0 < map.width && pos.1 > 0 && pos.1 < map.height {
            let offset = dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if map.get(new_pos.0, new_pos.1) == b'#' {
                dir = dir.turn_right();
            } else {
                pos = new_pos;
            }

            visited_map.set(pos.0, pos.1, b'X');
        }
    }

    let mut candidates = Vec::new();

    for y in 0..map.height {
        for x in 0..map.width {
            if visited_map.get(x, y) == b'X' && !(x == start_pos.0 && y == start_pos.1) {
                candidates.push((x, y))
            }
        }
    }

    let mut loops_count = 0;

    for candiadate in candidates {
        let mut map = map.clone();
        map.set(candiadate.0, candiadate.1, b'#');

        let mut visited_map = Map::empty(map.width, map.height);

        let mut dir = Dir::Up;
        let mut pos = start_pos;

        visited_map.set_or(start_pos.0, start_pos.1, dir.bits());

        while pos.0 > 0 && pos.0 < map.width && pos.1 > 0 && pos.1 < map.height {
            let offset = dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if map.get(new_pos.0, new_pos.1) == b'#' {
                dir = dir.turn_right();
            } else {
                pos = new_pos;
            }



            if (visited_map.get(pos.0, pos.1) & dir.bits()) > 0 {
                loops_count += 1;
                break;
            } else {
                visited_map.set_or(pos.0, pos.1, dir.bits());
            }
        }
    }

    println!("{}", !Dir::Up.bits());

    loops_count
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 6);
    assert_eq!(b(INPUT), 1697);
}
