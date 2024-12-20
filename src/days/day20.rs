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

pub fn djikstra_no_cheet(map: &Map<u8>, start: IVec2, end: IVec2) -> Option<u32> {
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

    let mut g_score = Map::<u32>::empty(map.width, map.height, u32::MAX);
    g_score.set(start, 0);

    let mut open_set = BinaryHeap::new();
    open_set.push(Cost {
        pos: start,
        cost: 0,
    });

    while let Some(Cost { pos: current, .. }) = open_set.pop() {
        if current == end {
            return Some(g_score.get(current));
        }

        for neighbor in [
            current + ivec2(1, 0),
            current + ivec2(-1, 0),
            current + ivec2(0, 1),
            current + ivec2(0, -1),
        ] {
            if map.get(neighbor) == b'#' {
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

pub fn djikstra_all_cheet(map: &Map<u8>, start: IVec2, end: IVec2, limit: u32) -> Option<i32> {
    #[derive(Clone, Copy, Default, Debug)]
    struct Pos {
        pos: IVec2,
        cheeted: i32,
        cost: u32,
    }

    impl PartialEq for Pos {
        fn eq(&self, other: &Self) -> bool {
            self.cost.eq(&other.cost)
        }
    }

    impl Eq for Pos {}

    impl PartialOrd for Pos {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Pos {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    struct PosMap<T>
    where
        T: Copy + Clone + MapDefault,
    {
        cheet_0_data: Vec<T>,
        cheet_1_data: Vec<T>,
        cheet_2_data: Vec<T>,
        width: i32,
        height: i32,
    }

    impl<T> PosMap<T>
    where
        T: Copy + Clone + MapDefault,
    {
        fn empty(width: i32, height: i32, initial: T) -> PosMap<T> {
            PosMap {
                cheet_0_data: vec![initial; (width * height) as usize],
                cheet_1_data: vec![initial; (width * height) as usize],
                cheet_2_data: vec![initial; (width * height) as usize],
                width,
                height,
            }
        }

        fn get(&self, pos: Pos) -> T {
            let data = match pos.cheeted {
                0 => self.cheet_0_data.as_slice(),
                1 => self.cheet_1_data.as_slice(),
                2 => self.cheet_2_data.as_slice(),
                _ => panic!("Bad state"),
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
            let data = match pos.cheeted {
                0 => self.cheet_0_data.as_mut_slice(),
                1 => self.cheet_1_data.as_mut_slice(),
                2 => self.cheet_2_data.as_mut_slice(),
                _ => panic!("Bad state"),
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

    let start = Pos {
        pos: start,
        cheeted: 0,
        cost: 0,
    };

    let mut g_score = PosMap::<u32>::empty(map.width, map.height, u32::MAX);
    g_score.set(start, 0);

    let mut open_set = BinaryHeap::new();
    open_set.push(start);

    let mut path_count = 0;

    let mut possible_neighbors = [Pos::default(); 8];

    while let Some(current) = open_set.pop() {
        if current.pos == end {
            println!("{} > {}", current.cost, limit);
            if current.cost > limit {
                return Some(path_count);
            }

            path_count += 1;
        }

        let neighbor_count = if current.cheeted == 0 {
            possible_neighbors.copy_from_slice(&[
                Pos {
                    pos: current.pos + ivec2(1, 0),
                    cheeted: 0,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(-1, 0),
                    cheeted: 0,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(0, 1),
                    cheeted: 0,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(0, -1),
                    cheeted: 0,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(1, 0),
                    cheeted: 1,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(-1, 0),
                    cheeted: 1,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(0, 1),
                    cheeted: 1,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(0, -1),
                    cheeted: 1,
                    cost: current.cost + 1,
                },
            ]);

            8
        } else {
            possible_neighbors[0..4].copy_from_slice(&[
                Pos {
                    pos: current.pos + ivec2(1, 0),
                    cheeted: 2,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(-1, 0),
                    cheeted: 2,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(0, 1),
                    cheeted: 2,
                    cost: current.cost + 1,
                },
                Pos {
                    pos: current.pos + ivec2(0, -1),
                    cheeted: 2,
                    cost: current.cost + 1,
                },
            ]);

            4
        };

        for neighbor in &possible_neighbors[..neighbor_count] {
            println!("{:?}", neighbor);

            if neighbor.cheeted != 1 && map.get(neighbor.pos) == b'#' {
                continue;
            }

            if neighbor.cost < limit && neighbor.cost < g_score.get(*neighbor) {
                g_score.set(*neighbor, neighbor.cost);
                open_set.push(*neighbor);
            }
        }
    }

    println!("End");

    Some(path_count)
}

pub fn a(input: &str, limit: u32) -> i32 {
    let mut map = Map::new(input);

    let start = map.find_first(b'S').unwrap();
    let end = map.find_first(b'E').unwrap();

    map.set(start, b'.');
    map.set(end, b'.');

    let shortest_no_cheet_path = djikstra_no_cheet(&map, start, end).unwrap();

    djikstra_all_cheet(
        &map,
        start,
        end,
        shortest_no_cheet_path.saturating_sub(limit),
    )
    .unwrap()
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT, 2), 14);
    //assert_eq!(a(INPUT, 100), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
