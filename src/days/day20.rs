use core::str;
use std::{cmp, collections::BinaryHeap, fmt::Display};

use glam::{ivec2, IVec2};

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
        b'.'
    }
}

impl MapDefault for u32 {
    fn map_default() -> Self {
        u32::MAX
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

        self.data[index as usize]
    }

    pub fn set(&mut self, pos: IVec2, new: T) -> bool {
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

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Pos {
    pos: IVec2,
    cheeted: bool,
}

impl Pos {
    fn new(pos: IVec2) -> Pos {
        Pos {
            pos,
            cheeted: false,
        }
    }
}

impl MapDefault for Pos {
    fn map_default() -> Self {
        Pos {
            pos: ivec2(0, 0),
            cheeted: false,
        }
    }
}

struct PosMap<T>
where
    T: Copy + Clone + MapDefault,
{
    data: Vec<T>,
    cheeted_data: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> PosMap<T>
where
    T: Copy + Clone + MapDefault,
{
    fn empty(width: i32, height: i32, initial: T) -> PosMap<T> {
        PosMap {
            data: vec![initial; (width * height) as usize],
            cheeted_data: vec![initial; (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, pos: Pos) -> T {
        let data = if pos.cheeted {
            self.cheeted_data.as_slice()
        } else {
            self.data.as_slice()
        };

        let pos = pos.pos;

        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return T::map_default();
        }

        if pos.y < 0 || pos.y >= self.height {
            return T::map_default();
        }

        data[index as usize]
    }

    fn set(&mut self, pos: Pos, new: T) -> bool {
        let data = if pos.cheeted {
            self.cheeted_data.as_mut_slice()
        } else {
            self.data.as_mut_slice()
        };

        let pos = pos.pos;
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return false;
        }

        if pos.y < 0 || pos.y >= self.height {
            return false;
        }

        data[index as usize] = new;

        true
    }
}

struct Cost {
    pos: Pos,
    cost: u32,
}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Cost {}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn djikstra_no_cheet(map: &Map<u8>, start: IVec2, end: IVec2) -> Option<i32> {
    let start = Pos {
        pos: start,
        cheeted: false,
    };

    let mut g_score = PosMap::<u32>::empty(map.width, map.height, u32::MAX);
    g_score.set(start, 0);

    let mut open_set = BinaryHeap::new();
    open_set.push(Cost {
        pos: start,
        cost: 0,
    });

    while let Some(Cost { pos: current, .. }) = open_set.pop() {
        if current.pos == end {
            return Some(g_score.get(current) as i32);
        }

        for neighbor in [
            Pos {
                pos: ivec2(1, 0),
                cheeted: false,
            },
            Pos {
                pos: ivec2(-1, 0),
                cheeted: false,
            },
            Pos {
                pos: ivec2(0, 1),
                cheeted: false,
            },
            Pos {
                pos: ivec2(0, -1),
                cheeted: false,
            },
        ] {
            if map.get(neighbor.pos) == b'#' {
                continue;
            }

            let tentative_g_score = g_score.get(current) + 1;
            if tentative_g_score < g_score.get(neighbor) {
                g_score.set(neighbor, tentative_g_score);

                open_set.push(Cost {
                    pos: neighbor,
                    cost: tentative_g_score,
                });
            }
        }
    }

    None
}

pub fn a(input: &str) -> i32 {
    let mut map = Map::new(input);

    let start = map.find_first(b'S').unwrap();
    let end = map.find_first(b'E').unwrap();

    map.set(start, b'.');
    map.set(end, b'.');

    let shortest_no_cheet_path = djikstra_no_cheet(&map, start, end).unwrap();

    0
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 100);
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
