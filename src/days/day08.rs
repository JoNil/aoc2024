use std::{fmt::Display, str};

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

    println!("{}", map);

    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 14);
    assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
