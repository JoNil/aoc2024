use glam::{ivec2, IVec2};
use std::collections::{HashMap, HashSet};

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
}

fn flood(
    processed_positions: &mut HashSet<IVec2>,
    map: &Map,
    region: &mut Vec<IVec2>,
    p: IVec2,
    c: u8,
) {
    if !processed_positions.contains(&p) && map.get(p) == c {
        region.push(p);
        processed_positions.insert(p);

        for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            flood(processed_positions, map, region, p + dir, c);
        }
    }
}

pub fn a(input: &str) -> i32 {
    let map = Map::new(input);
    let mut processed_positions = HashSet::new();
    let mut regions = HashMap::<u8, Vec<Vec<IVec2>>>::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let p = ivec2(x, y);
            let c = map.get(p);

            if !processed_positions.contains(&p) {
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

pub fn b(input: &str) -> i32 {
    let map = Map::new(input);
    let mut processed_positions = HashSet::new();
    let mut regions = HashMap::<u8, Vec<Vec<IVec2>>>::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let p = ivec2(x, y);
            let c = map.get(p);

            if !processed_positions.contains(&p) {
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

            let region_pices = region.iter().copied().collect::<HashSet<_>>();

            let mut borderpices = Vec::new();

            for pice in region {
                let mut neighbour_count = 0;

                for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                    if map.get(*pice + dir) == *label {
                        neighbour_count += 1;
                    }
                }
            }

            price += area * border;
        }
    }

    price as i32
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1260);
    assert_eq!(b(INPUT), 0);
}
