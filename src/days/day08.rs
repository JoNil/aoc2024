use glam::{ivec2, IVec2};
use std::{collections::HashMap, fmt::Display, str};

pub static INPUT: &str = include_str!("../input/8.txt");
pub static TEST_INPUT: &str = include_str!("../input/8_test.txt");

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
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            writeln!(f, "{}", str::from_utf8(line).unwrap())?;
        }

        Ok(())
    }
}

pub fn a(input: &str) -> i32 {
    let map = Map::new(input);

    let mut antennas = HashMap::<u8, Vec<IVec2>>::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let v = map.get(x, y);
            if v != b'.' {
                antennas.entry(v).or_default().push(ivec2(x, y));
            }
        }
    }

    let mut antinode_map = Map::empty(map.width, map.height);

    for antennas in antennas.values() {
        for a in 0..antennas.len() {
            for b in (a + 1)..antennas.len() {
                let a = antennas[a];
                let b = antennas[b];

                let diff = a - b;

                let antinode = a + diff;
                antinode_map.set(antinode.x, antinode.y, b'#');

                let antinode = b - diff;
                antinode_map.set(antinode.x, antinode.y, b'#');
            }
        }
    }

    antinode_map.data.iter().filter(|v| **v == b'#').count() as _
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 14);
    assert_eq!(a(INPUT), 259);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
