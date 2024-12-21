use std::{cmp, collections::BinaryHeap, str};

use cached::proc_macro::cached;
use glam::{ivec2, IVec2};
use smallvec::SmallVec;

pub static INPUT: &str = include_str!("../input/21.txt");
pub static TEST_INPUT: &str = include_str!("../input/21_test.txt");

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

struct CameFrom {
    data: Vec<SmallVec<[IVec2; 2]>>,
    width: i32,
    height: i32,
}

impl CameFrom {
    fn empty(width: i32, height: i32) -> CameFrom {
        CameFrom {
            data: vec![SmallVec::new(); (width * height) as usize],
            width,
            height,
        }
    }

    fn get_mut(&mut self, pos: IVec2) -> &mut SmallVec<[IVec2; 2]> {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            panic!();
        }

        if pos.y < 0 || pos.y >= self.height {
            panic!();
        }

        &mut self.data[index as usize]
    }
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

fn all_paths(came_from: &mut CameFrom, current: IVec2, start: IVec2) -> Vec<Vec<IVec2>> {
    let mut paths = Vec::new();
    paths.push(vec![current]);

    loop {
        let mut new_paths = Vec::new();

        let mut did_work = false;

        for path in &mut paths {
            let current = path.last().unwrap();

            if *current == start {
                continue;
            }

            did_work = true;
            let came_from = came_from.get_mut(*current);

            #[allow(clippy::comparison_chain)]
            if came_from.len() == 1 {
                path.push(came_from[0]);
            } else if came_from.len() > 1 {
                let old = path.clone();

                for (i, next) in came_from.iter().enumerate() {
                    if i == 0 {
                        path.push(*next);
                    } else {
                        let mut new_path = old.clone();
                        new_path.push(*next);
                        new_paths.push(new_path);
                    }
                }
            }
        }

        for new_path in new_paths {
            paths.push(new_path);
        }

        if !did_work {
            break;
        }
    }

    for path in &mut paths {
        path.reverse();
    }

    paths
}

fn pos_from_digit(digit: u8) -> IVec2 {
    match digit {
        b'7' => ivec2(0, 0),
        b'8' => ivec2(1, 0),
        b'9' => ivec2(2, 0),

        b'4' => ivec2(0, 1),
        b'5' => ivec2(1, 1),
        b'6' => ivec2(2, 1),

        b'1' => ivec2(0, 2),
        b'2' => ivec2(1, 2),
        b'3' => ivec2(2, 2),

        b'0' => ivec2(1, 3),
        b'A' => ivec2(2, 3),
        _ => panic!("Invalid"),
    }
}

fn pos_from_dir(dir: IVec2) -> IVec2 {
    match dir {
        IVec2 { x: 1, y: 0 } => ivec2(2, 1),
        IVec2 { x: -1, y: 0 } => ivec2(0, 1),
        IVec2 { x: 0, y: 1 } => ivec2(1, 0),
        IVec2 { x: 0, y: -1 } => ivec2(1, 1),

        _ => panic!("Invalid"),
    }
}

#[cached]
fn path_keypad(start: IVec2, end: IVec2) -> Vec<Vec<IVec2>> {
    let mut g_score = Map::<u32>::empty(3, 4, u32::MAX);
    g_score.set(start, 0);

    let mut open_set = BinaryHeap::new();
    open_set.push(Cost {
        pos: start,
        cost: 0,
    });

    let mut came_from = CameFrom::empty(3, 4);

    while let Some(Cost { pos: current, .. }) = open_set.pop() {
        if current == end {
            return all_paths(&mut came_from, current, start);
        }

        for neighbor_dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
            let neighbor = current + neighbor_dir;

            if !((0..3).contains(&neighbor.x)
                && (0..4).contains(&neighbor.y)
                && neighbor != ivec2(0, 3))
            {
                continue;
            }

            let tentative_g_score = g_score.get(current) + 1;
            let neighbor_g_score = g_score.get(neighbor);
            if tentative_g_score <= neighbor_g_score {
                let came_from = came_from.get_mut(neighbor);
                if tentative_g_score < neighbor_g_score {
                    came_from.clear();
                }
                came_from.push(current);

                if tentative_g_score < neighbor_g_score {
                    g_score.set(neighbor, tentative_g_score);

                    open_set.push(Cost {
                        pos: neighbor,
                        cost: tentative_g_score,
                    });
                }
            }
        }
    }

    panic!("No path");
}

fn map_pair_to_dir(a: IVec2, b: IVec2) -> u8 {
    match b - a {
        IVec2 { x: 1, y: 0 } => b'>',
        IVec2 { x: -1, y: 0 } => b'<',
        IVec2 { x: 0, y: 1 } => b'^',
        IVec2 { x: 0, y: -1 } => b'v',
        _ => panic!("Bad"),
    }
}

fn find_shortest_sequence(code: &[u8]) -> i32 {
    let mut start = b'A';
    for &end in code {
        println!("{} -> {}", start as char, end as char);
        let paths = path_keypad(pos_from_digit(start), pos_from_digit(end));
        println!("{paths:#?}");
        start = end;
    }
    0
}

pub fn a(input: &str) -> i32 {
    let codes = input
        .lines()
        .map(|s| {
            (
                s.as_bytes(),
                s.trim_end_matches('A').parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum_of_complexity = 0;

    for (code, code_no) in codes {
        let complexity = find_shortest_sequence(code);
        sum_of_complexity += code_no * complexity;
    }

    sum_of_complexity
}

#[test]
fn test_a() {
    assert_eq!(a("029A"), 1);
    assert_eq!(a(TEST_INPUT), 126384);
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
