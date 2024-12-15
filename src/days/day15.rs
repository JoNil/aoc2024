use std::{fmt::Display, str};

use glam::{ivec2, IVec2};

pub static INPUT: &str = include_str!("../input/15.txt");
pub static TEST_INPUT: &str = include_str!("../input/15_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/15_test_2.txt");

struct Map {
    data: Vec<u8>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(input: &str) -> Map {
        let data = input.replace('\n', "").into_bytes();

        let mut width: i32 = 0;

        if let Some(line) = input.lines().next() {
            width = line.len() as i32;
        }

        let height = data.len() as i32 / width;

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

    fn find_first(&self, needle: u8) -> Option<IVec2> {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = ivec2(x, y);
                let v = self.get(pos);

                if v == needle {
                    return Some(pos);
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

fn resolve_collision(map: &mut Map, pos: IVec2, dir: IVec2, payload: bool) -> bool {
    let v = map.get(pos);

    match v {
        b'#' => false,
        b'.' => {
            if payload {
                map.set(pos, b'O');
            }
            true
        }
        b'O' => {
            let next_ok = resolve_collision(map, pos + dir, dir, true);

            if next_ok {
                if payload {
                    map.set(pos, b'O');
                } else {
                    map.set(pos, b'.');
                }
            }

            next_ok
        }
        _ => panic!("Bad map char"),
    }
}

pub fn a(input: &str) -> i32 {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let mut map = Map::new(map);
    let mut pos = map.find_first(b'@').unwrap();
    map.set(pos, b'.');

    for instruction in instructions.as_bytes() {
        let dir = match *instruction {
            b'<' => ivec2(-1, 0),
            b'>' => ivec2(1, 0),
            b'^' => ivec2(0, -1),
            b'v' => ivec2(0, 1),
            _ => continue,
        };

        let new_pos = pos + dir;
        let ok_move = resolve_collision(&mut map, new_pos, dir, false);

        if ok_move {
            pos = new_pos;
        }
    }

    let mut score = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(ivec2(x, y)) == b'O' {
                score += 100 * y + x;
            }
        }
    }

    score
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT_2), 2028);
    assert_eq!(a(TEST_INPUT), 10092);
    assert_eq!(a(INPUT), 1509074);
}

fn resolve_collision_b(map: &mut Map, pos: IVec2, dir: IVec2, payload: bool) -> bool {
    let v = map.get(pos);

    match v {
        b'#' => false,
        b'.' => {
            if payload {
                map.set(pos, b'O');
            }
            true
        }
        b'O' => {
            let next_ok = resolve_collision(map, pos + dir, dir, true);

            if next_ok {
                if payload {
                    map.set(pos, b'O');
                } else {
                    map.set(pos, b'.');
                }
            }

            next_ok
        }
        _ => panic!("Bad map char"),
    }
}

pub fn b(input: &str) -> i32 {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let original_map = Map::new(map);

    let mut map = Map::empty(original_map.width * 2, original_map.height);

    let mut pos = ivec2(0, 0);

    for y in 0..original_map.height {
        for x in 0..original_map.height {
            let v = original_map.get(ivec2(x, y));

            match v {
                b'#' => {
                    map.set(ivec2(2 * x, y), b'#');
                    map.set(ivec2(2 * x + 1, y), b'#');
                }
                b'O' => {
                    map.set(ivec2(2 * x, y), b'[');
                    map.set(ivec2(2 * x + 1, y), b']');
                }
                b'@' => pos = ivec2(2 * x, y),
                _ => continue,
            }
        }
    }

    println!("{}", map);

    for instruction in instructions.as_bytes() {
        let dir = match *instruction {
            b'<' => ivec2(-1, 0),
            b'>' => ivec2(1, 0),
            b'^' => ivec2(0, -1),
            b'v' => ivec2(0, 1),
            _ => continue,
        };

        let new_pos = pos + dir;
        let ok_move = resolve_collision_b(&mut map, new_pos, dir, false);

        if ok_move {
            pos = new_pos;
        }
    }

    let mut score = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(ivec2(x, y)) == b'O' {
                score += 100 * y + x;
            }
        }
    }

    score
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 9021);
    assert_eq!(b(INPUT), 0);
}
