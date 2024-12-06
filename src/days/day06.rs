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

    fn get(&self, index: i32) -> u8 {
        unsafe { *self.data.get_unchecked(index as usize) }
    }

    fn set(&mut self, index: i32, new: u8) {
        unsafe {
            *self.data.get_unchecked_mut(index as usize) = new;
        }
    }

    fn set_or(&mut self, index: i32, new: u8) {
        unsafe {
            *self.data.get_unchecked_mut(index as usize) |= new;
        }
    }

    fn find_first(&self, needle: u8) -> Option<(i32, i32)> {
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.get(x + self.width * y);

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

#[derive(Clone, Copy)]
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

    map.set(pos.0 + map.width * pos.1, b'.');
    visited_map.set(pos.0 + map.width * pos.1, b'X');

    while pos.0 > 0 && pos.0 < map.width - 1 && pos.1 > 0 && pos.1 < map.height - 1 {
        let offset = dir.offset();
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

        if map.get(new_pos.0 + map.width * new_pos.1) == b'#' {
            dir = dir.turn_right();
        } else {
            pos = new_pos;
        }

        visited_map.set(pos.0 + map.width * pos.1, b'X');
    }

    visited_map.data.iter().filter(|v| **v == b'X').count() as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 41);
    assert_eq!(a(INPUT), 4988);
}

pub fn b(input: &str) -> i32 {
    let (mut map, start_pos) = {
        let mut map = Map::new(input);

        let start_pos = map.find_first(b'^').unwrap();

        map.set(start_pos.0 + map.width * start_pos.1, b'.');

        (map, start_pos)
    };

    let mut visited_map = Map::empty(map.width, map.height);
    let mut new_visited_map = Map::empty(map.width, map.height);
    let mut loops_count = 0;

    {
        let mut dir = Dir::Up;
        let mut pos = start_pos;

        while pos.0 > 0 && pos.0 < map.width - 1 && pos.1 > 0 && pos.1 < map.height - 1 {
            let offset = dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
            let mut index = new_pos.0 + map.width * new_pos.1;

            if map.get(new_pos.0 + map.width * new_pos.1) == b'#' {
                dir = dir.turn_right();
                index = pos.0 + map.width * pos.1;
            } else {
                pos = new_pos;
            }

            if visited_map.get(index) == 0 {
                let candiadate_x = pos.0;
                let candiadate_y = pos.1;
                let candidate_dir = dir;

                map.set(candiadate_x + map.width * candiadate_y, b'#');

                let candidate_offset = candidate_dir.offset();

                let mut dir = candidate_dir;
                let mut pos = (
                    candiadate_x - candidate_offset.0,
                    candiadate_y - candidate_offset.1,
                );

                new_visited_map.data.copy_from_slice(&visited_map.data);
                new_visited_map.set_or(pos.0 + map.width * pos.1, dir.bits());

                while pos.0 > 0 && pos.0 < map.width - 1 && pos.1 > 0 && pos.1 < map.height - 1 {
                    let offset = dir.offset();
                    let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
                    let mut index = new_pos.0 + map.width * new_pos.1;

                    if map.get(index) == b'#' {
                        dir = dir.turn_right();
                        index = pos.0 + map.width * pos.1;
                    } else {
                        pos = new_pos;
                    }

                    if (new_visited_map.get(index) & dir.bits()) > 0 {
                        loops_count += 1;
                        break;
                    } else {
                        new_visited_map.set_or(index, dir.bits());
                    }
                }

                map.set(candiadate_x + map.width * candiadate_y, b'.');
            }

            visited_map.set_or(index, dir.bits());
        }
    }

    loops_count
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 6);
    assert_eq!(b(INPUT), 1697);
}
