use glam::{ivec2, IVec2};
use std::{collections::HashSet, fmt::Display, str};

pub static INPUT: &str = include_str!("../input/10.txt");
pub static TEST_INPUT: &str = include_str!("../input/10_test.txt");

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

    fn ascii_digits_to_u8(mut self) -> Map {
        for value in self.data.iter_mut() {
            *value -= 48;
        }

        self
    }

    fn get(&self, pos: IVec2) -> u8 {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return Default::default();
        }

        if pos.y < 0 || pos.y >= self.height {
            return Default::default();
        }

        self.data[index as usize]
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

fn find_paths(map: &Map, reachable_nines: &mut Vec<HashSet<IVec2>>, pos: IVec2) -> HashSet<IVec2> {
    {
        let cached = reachable_nines
            .get((pos.x + pos.y * map.width) as usize)
            .unwrap();
        if cached.len() > 0 {
            return cached.clone();
        }
    }

    let value = map.get(pos);
    if value == 9 {
        let mut res = HashSet::new();
        res.insert(pos);
        return res;
    }

    let mut possible_paths = HashSet::new();

    for possible_dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
        let candidate_pos = pos + possible_dir;
        let candidate_val = map.get(candidate_pos);

        if candidate_val as i32 - value as i32 == 1 {
            let new_paths = find_paths(map, reachable_nines, candidate_pos);
            possible_paths = possible_paths
                .union(&new_paths)
                .copied()
                .collect::<HashSet<_>>();
        }
    }

    reachable_nines[(pos.x + pos.y * map.width) as usize] = possible_paths.clone();

    possible_paths
}

pub fn a(input: &str) -> i32 {
    let map = Map::new(input).ascii_digits_to_u8();
    let mut reachable_nines = vec![HashSet::new(); (map.width * map.height) as usize];

    let mut sum_of_reachable = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = ivec2(x, y);
            let val = map.get(pos);

            if val == 0 {
                let nines_reachable = find_paths(&map, &mut reachable_nines, pos);
                sum_of_reachable += nines_reachable.len() as i32;
            }
        }
    }

    sum_of_reachable
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 36);
    assert_eq!(a(INPUT), 688);
}

fn find_paths_b(map: &Map, reachable_nines: &mut Vec<i32>, pos: IVec2) -> i32 {
    {
        let cached = reachable_nines
            .get((pos.x + pos.y * map.width) as usize)
            .unwrap();
        if *cached > 0 {
            return *cached;
        }
    }

    let value = map.get(pos);
    if value == 9 {
        return 1;
    }

    let mut possible_paths = 0;

    for possible_dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
        let candidate_pos = pos + possible_dir;
        let candidate_val = map.get(candidate_pos);

        if candidate_val as i32 - value as i32 == 1 {
            possible_paths += find_paths_b(map, reachable_nines, candidate_pos);
        }
    }

    reachable_nines[(pos.x + pos.y * map.width) as usize] = possible_paths;

    possible_paths
}

pub fn b(input: &str) -> i32 {
    let map = Map::new(input).ascii_digits_to_u8();
    let mut reachable_nines = vec![0; (map.width * map.height) as usize];

    let mut sum_of_reachable = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = ivec2(x, y);
            let val = map.get(pos);

            if val == 0 {
                sum_of_reachable += find_paths_b(&map, &mut reachable_nines, pos);
            }
        }
    }

    sum_of_reachable
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 81);
    assert_eq!(b(INPUT), 1459);
}
