use glam::{ivec2, IVec2};
use std::{collections::HashSet, fmt::Display, str};

pub static INPUT: &str = include_str!("../input/10.txt");
pub static TEST_INPUT: &str = include_str!("../input/10_test.txt");

#[derive(Clone)]
struct Map<T>
where
    T: Copy + Clone + Default,
{
    data: Vec<T>,
    width: i32,
    height: i32,
}

impl Map<u8> {
    fn new(input: &str) -> Map<u8> {
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

    fn ascii_digits_to_u8(mut self) -> Map<u8> {
        for value in self.data.iter_mut() {
            *value -= 48;
        }

        self
    }
}

impl<T> Map<T>
where
    T: Copy + Clone + Default,
{
    fn empty(width: i32, height: i32) -> Map<T> {
        Map {
            data: vec![Default::default(); (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, pos: IVec2) -> T {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return Default::default();
        }

        if pos.y < 0 || pos.y >= self.height {
            return Default::default();
        }

        self.data[index as usize]
    }

    fn set(&mut self, pos: IVec2, new: T) -> bool {
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

impl Display for Map<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            writeln!(f, "{}", str::from_utf8(line).unwrap())?;
        }

        Ok(())
    }
}

fn find_paths(map: &Map<u8>, reachable_nines: &mut Map<i32>, pos: IVec2) -> HashSet<IVec2> {
    /*let cached = reachable_nines.get(pos);
    if cached > 0 {
        return cached;
    }*/

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

    /*if cached == 0 {
        reachable_nines.set(pos, possible_paths);
    }*/

    possible_paths
}

pub fn a(input: &str) -> i32 {
    let map = Map::new(input).ascii_digits_to_u8();
    let mut reachable_nines = Map::empty(map.width, map.height);

    let mut sum_of_reachable = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            let pos = ivec2(x, y);
            let val = map.get(pos);

            if val == 0 {
                let nines_reachable = find_paths(&map, &mut reachable_nines, pos);

                println!("{}", nines_reachable.len());

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

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
