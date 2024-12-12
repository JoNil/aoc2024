use glam::{ivec2, IVec2};
use std::{collections::HashMap, fmt::Display, str};

pub static INPUT: &str = include_str!("../input/12.txt");
pub static TEST_INPUT: &str = include_str!("../input/12_test.txt");

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

fn flood(processed_positions: &mut Map, map: &Map, region: &mut Vec<IVec2>, p: IVec2, c: u8) {
    if processed_positions.get(p) == b'.' && map.get(p) == c {
        region.push(p);
        processed_positions.set(p, b'x');

        for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            flood(processed_positions, map, region, p + dir, c);
        }
    }
}

pub fn a(input: &str) -> i32 {
    let map = Map::new(input);
    let mut processed_positions = Map::empty(map.width, map.height);
    let mut regions = HashMap::<u8, Vec<Vec<IVec2>>>::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let p = ivec2(x, y);
            let c = map.get(p);

            if processed_positions.get(p) == b'.' {
                let mut region = Vec::new();
                flood(&mut processed_positions, &map, &mut region, p, c);
                regions.entry(c).or_default().push(region);
            }
        }
    }

    let mut price = 0;

    for (label, regions) in &regions {
        for region in regions {
            let area = region.len();
            let mut border = 0;

            for pos in region {
                for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                    if map.get(pos + dir) != *label {
                        border += 1;
                    }
                }
            }

            price += area * border;
        }
    }

    price as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1930);
    assert_eq!(a(INPUT), 1431316);
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn fwd(self) -> IVec2 {
        match self {
            Dir::Up => ivec2(0, -1),
            Dir::Down => ivec2(0, 1),
            Dir::Left => ivec2(-1, 0),
            Dir::Right => ivec2(1, 0),
        }
    }

    fn right(self) -> IVec2 {
        match self {
            Dir::Up => ivec2(1, 0),
            Dir::Down => ivec2(-1, 0),
            Dir::Left => ivec2(0, 1),
            Dir::Right => ivec2(0, -1),
        }
    }
}

pub fn b(input: &str) -> i32 {
    let map = Map::new(input);
    let mut processed_positions = Map::empty(map.width, map.height);
    let mut regions = HashMap::<u8, Vec<Vec<IVec2>>>::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let p = ivec2(x, y);
            let c = map.get(p);

            if processed_positions.get(p) == b'.' {
                let mut region = Vec::new();
                flood(&mut processed_positions, &map, &mut region, p, c);
                regions.entry(c).or_default().push(region);
            }
        }
    }

    let mut price = 0;

    for (label, regions) in &regions {
        for region in regions {
            let area = region.len();
            let mut border = 0;

            let mut borderpices = Vec::new();

            for pice in region {
                let mut neighbour_count = 0;

                for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                    if map.get(*pice + dir) == *label {
                        neighbour_count += 1;
                    }
                }

                if neighbour_count != 4 {
                    borderpices.push(pice);
                }
            }

            price += area * border;

            let mut border_map = Map::empty(map.width, map.height);

            for pice in borderpices {
                border_map.set(*pice, *label);
            }

            println!("{}", map);
            println!("{}", border_map);
        }
    }

    price as i32
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1206);
    assert_eq!(b(INPUT), 0);
}
