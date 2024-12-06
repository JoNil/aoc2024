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
            data: vec![b'.'; (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, x: i32, y: i32) -> u8 {
        let index = x + y * self.width;

        if x < 0 || x >= self.width {
            return b'.';
        }

        if y < 0 || y >= self.height {
            return b'.';
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

        let mut visited_map_r = Map::empty(map.width, map.height);
        let mut visited_map_l = Map::empty(map.width, map.height);
        let mut visited_map_u = Map::empty(map.width, map.height);
        let mut visited_map_d = Map::empty(map.width, map.height);

        let mut dir = Dir::Up;
        let mut pos = start_pos;

        visited_map_u.set(start_pos.0, start_pos.1, b'X');

        while pos.0 > 0 && pos.0 < map.width && pos.1 > 0 && pos.1 < map.height {
            let offset = dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if map.get(new_pos.0, new_pos.1) == b'#' {
                dir = dir.turn_right();
            } else {
                pos = new_pos;
            }

            match dir {
                Dir::Up => {
                    if visited_map_u.get(pos.0, pos.1) == b'X' {
                        loops_count += 1;
                        break;
                    } else {
                        visited_map_u.set(pos.0, pos.1, b'X');
                    }
                }
                Dir::Right => {
                    if visited_map_r.get(pos.0, pos.1) == b'X' {
                        loops_count += 1;
                        break;
                    } else {
                        visited_map_r.set(pos.0, pos.1, b'X');
                    }
                }
                Dir::Down => {
                    if visited_map_d.get(pos.0, pos.1) == b'X' {
                        loops_count += 1;
                        break;
                    } else {
                        visited_map_d.set(pos.0, pos.1, b'X');
                    }
                }
                Dir::Left => {
                    if visited_map_l.get(pos.0, pos.1) == b'X' {
                        loops_count += 1;
                        break;
                    } else {
                        visited_map_l.set(pos.0, pos.1, b'X');
                    }
                }
            }
        }
    }

    loops_count
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 6);
    assert_eq!(b(INPUT), 1697);
}
