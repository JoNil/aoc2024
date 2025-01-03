use glam::{ivec2, IVec2};
use std::{fmt::Display, str};

pub static INPUT: &str = include_str!("../input/20.txt");
pub static TEST_INPUT: &str = include_str!("../input/20_test.txt");

pub trait MapDefault {
    fn map_default() -> Self;
}

impl MapDefault for bool {
    fn map_default() -> Self {
        false
    }
}

impl MapDefault for u8 {
    fn map_default() -> Self {
        b'#'
    }
}

impl MapDefault for u32 {
    fn map_default() -> Self {
        0
    }
}

impl MapDefault for IVec2 {
    fn map_default() -> Self {
        ivec2(0, 0)
    }
}

#[derive(Clone)]
pub struct Map<T>
where
    T: Copy + Clone + MapDefault,
{
    data: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> Map<T>
where
    T: Copy + Clone + MapDefault,
{
    pub fn empty(width: i32, height: i32, initial: T) -> Map<T> {
        Map {
            data: vec![initial; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get(&self, pos: IVec2) -> T {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return T::map_default();
        }

        if pos.y < 0 || pos.y >= self.height {
            return T::map_default();
        }

        unsafe { *self.data.get_unchecked(index as usize) }
    }

    pub fn set(&mut self, pos: IVec2, new: T) -> bool {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return false;
        }

        if pos.y < 0 || pos.y >= self.height {
            return false;
        }

        unsafe {
            *self.data.get_unchecked_mut(index as usize) = new;
        }

        true
    }
}

impl Map<u8> {
    pub fn new(input: &str) -> Map<u8> {
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

    pub fn find_first(&self, needle: u8) -> Option<IVec2> {
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

impl Display for Map<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            writeln!(f, "{}", str::from_utf8(line).unwrap())?;
        }

        Ok(())
    }
}

impl Display for Map<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            for char in line {
                if *char == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", ((char % 10) + 48) as u8 as char)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Display for Map<IVec2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            for dir in line {
                match dir {
                    IVec2 { x: 1, y: 0 } => write!(f, ">")?,
                    IVec2 { x: -1, y: 0 } => write!(f, "<")?,
                    IVec2 { x: 0, y: 1 } => write!(f, "v")?,
                    IVec2 { x: 0, y: -1 } => write!(f, "^")?,
                    IVec2 { x: 0, y: 0 } => write!(f, ".")?,
                    _ => panic!("Unexpected"),
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn a(input: &str, limit: u32) -> i32 {
    let mut map = Map::new(input);

    let start = map.find_first(b'S').unwrap();
    let end = map.find_first(b'E').unwrap();

    map.set(start, b'.');
    map.set(end, b'.');

    let mut path_map = Map::empty(map.width, map.height, 0u32);
    let mut path = Vec::with_capacity((map.width * map.height) as usize / 2);
    let mut length = 0;

    {
        let mut current = start;
        let mut last = start;

        loop {
            if current == end {
                path_map.set(current, length + 1);
                path.push((current, length + 1));
                break;
            }

            for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                let next = current + dir;

                if next == last || map.get(next) == b'#' {
                    continue;
                }

                path_map.set(current, length);
                path.push((current, length));

                last = current;
                current = next;
                length += 1;
            }
        }
    }

    let mut possible_skips = 0;

    for (pos, pos_count) in path {
        for skip_dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let skip_pos = pos + skip_dir;
            if map.get(skip_pos) == b'#' {
                let skip_count = path_map.get(pos + 2 * skip_dir);
                if skip_count > 0 && skip_count as i32 - pos_count as i32 > limit as i32 {
                    possible_skips += 1;
                }
            }
        }
    }

    possible_skips
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT, 2), 44);
    assert_eq!(a(TEST_INPUT, 4), 30);
    assert_eq!(a(TEST_INPUT, 64), 1);
    assert_eq!(a(INPUT, 100), 1358);
}

fn manhattan_iter(dist: i32) -> impl Iterator<Item = IVec2> {
    (-dist..=dist).flat_map(move |x| {
        let max_y = dist - x.abs();
        (-max_y..=max_y)
            .map(move |y| IVec2::new(x, y))
            .filter(|p| (p.x.abs() + p.y.abs()) > 1)
    })
}

pub fn b(input: &str, limit: u32) -> i32 {
    let mut map = Map::new(input);

    let start = map.find_first(b'S').unwrap();
    let end = map.find_first(b'E').unwrap();

    map.set(start, b'.');
    map.set(end, b'.');

    let mut path_map = Map::empty(map.width, map.height, 0u32);
    let mut path = Vec::with_capacity((map.width * map.height) as usize / 2);
    let mut length = 0;

    {
        let mut current = start;
        let mut last = start;

        loop {
            if current == end {
                path_map.set(current, length + 1);
                path.push((current, length + 1));
                break;
            }

            for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                let next = current + dir;

                if next == last || map.get(next) == b'#' {
                    continue;
                }

                path_map.set(current, length);
                path.push((current, length));

                last = current;
                current = next;
                length += 1;
            }
        }
    }

    let mut possible_skips = 0;

    let offsets = manhattan_iter(20)
        .map(|dir| (dir, dir.x.abs() + dir.y.abs()))
        .collect::<Vec<_>>();

    for (pos, pos_count) in path {
        for (skip_dir, skip_dist) in &offsets {
            let skip_pos = pos + skip_dir;
            let skip_count = path_map.get(skip_pos);

            let skip_len = skip_count as i32 - pos_count as i32 - skip_dist;

            if skip_count > 0 && skip_len >= limit as i32 {
                possible_skips += 1;
            }
        }
    }

    possible_skips
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT, 50), 285);
    assert_eq!(b(INPUT, 100), 1005856);
}
