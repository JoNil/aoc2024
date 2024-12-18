use glam::{ivec2, IVec2};
use std::{cmp, collections::BinaryHeap, fmt::Display, str};

pub static INPUT: &str = include_str!("../input/18.txt");
pub static TEST_INPUT: &str = include_str!("../input/18_test.txt");

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

fn manhattan(a: IVec2, b: IVec2) -> u32 {
    (a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()
}

struct Cost {
    pos: IVec2,
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

pub fn a(input: &str, size: IVec2, steps: i32) -> i32 {
    let mut map = Map::empty(size.x, size.y, b'.');

    for (step, line) in input.lines().enumerate() {
        if step as i32 == steps {
            break;
        }

        let (x, y) = line.split_once(',').unwrap();
        let pos = ivec2(x.parse().unwrap(), y.parse().unwrap());
        map.set(pos, b'#');
    }

    let start = ivec2(0, 0);
    let end = ivec2(size.x - 1, size.y - 1);

    let start_cost = manhattan(start, end);

    let mut g_score = Map::<u32>::empty(map.width, map.height, u32::MAX);
    g_score.set(start, 0);

    let mut f_score = Map::<u32>::empty(map.width, map.height, u32::MAX);
    f_score.set(start, start_cost);

    let mut open_set = BinaryHeap::new();
    open_set.push(Cost {
        pos: start,
        cost: start_cost,
    });

    while let Some(Cost { pos: current, .. }) = open_set.pop() {
        if current == end {
            return g_score.get(current) as i32;
        }

        for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let neighbor = current + dir;

            if map.get(neighbor) == b'#' {
                continue;
            }

            let tentative_g_score = g_score.get(current) + 1;
            if tentative_g_score < g_score.get(neighbor) {
                let neighbor_f_score = tentative_g_score + manhattan(neighbor, end);

                g_score.set(neighbor, tentative_g_score);
                f_score.set(neighbor, neighbor_f_score);

                open_set.push(Cost {
                    pos: neighbor,
                    cost: neighbor_f_score,
                });
            }
        }
    }

    panic!("No path");
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT, ivec2(7, 7), 12), 22);
    assert_eq!(a(INPUT, ivec2(71, 71), 1024), 0);
}

pub fn b(input: &str, size: IVec2) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT, ivec2(7, 7)), 0);
    assert_eq!(b(INPUT, ivec2(70, 70)), 0);
}
