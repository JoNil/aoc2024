use glam::{ivec2, IVec2};
use std::{cmp, collections::BinaryHeap, fmt::Display, str};

pub static INPUT: &str = include_str!("../input/16.txt");
pub static TEST_INPUT: &str = include_str!("../input/16_test.txt");
pub static TEST_INPUT_2: &str = include_str!("../input/16_test_2.txt");

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

#[derive(Copy, Clone, Default, PartialEq, Eq, Debug)]
enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn fwd(&self) -> IVec2 {
        match self {
            Dir::Up => ivec2(0, -1),
            Dir::Down => ivec2(0, 1),
            Dir::Left => ivec2(-1, 0),
            Dir::Right => ivec2(1, 0),
        }
    }

    fn turn_right(self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }

    fn turn_left(self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }
}

fn manhattan(a: IVec2, b: IVec2) -> u32 {
    (a.x - b.x).unsigned_abs() + (a.y - b.y).unsigned_abs()
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Pos {
    pos: IVec2,
    dir: Dir,
}

impl Pos {
    fn new(pos: IVec2, dir: Dir) -> Pos {
        Pos { pos, dir }
    }
}

impl MapDefault for Pos {
    fn map_default() -> Self {
        Pos {
            pos: ivec2(0, 0),
            dir: Dir::Up,
        }
    }
}

struct PosMap<T>
where
    T: Copy + Clone + MapDefault,
{
    up_data: Vec<T>,
    down_data: Vec<T>,
    left_data: Vec<T>,
    right_data: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> PosMap<T>
where
    T: Copy + Clone + MapDefault,
{
    fn empty(width: i32, height: i32, initial: T) -> PosMap<T> {
        PosMap {
            up_data: vec![initial; (width * height) as usize],
            down_data: vec![initial; (width * height) as usize],
            left_data: vec![initial; (width * height) as usize],
            right_data: vec![initial; (width * height) as usize],
            width,
            height,
        }
    }

    fn get(&self, pos: Pos) -> T {
        let data = match pos.dir {
            Dir::Up => self.up_data.as_slice(),
            Dir::Down => self.down_data.as_slice(),
            Dir::Left => self.left_data.as_slice(),
            Dir::Right => self.right_data.as_slice(),
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
        let data = match pos.dir {
            Dir::Up => self.up_data.as_mut_slice(),
            Dir::Down => self.down_data.as_mut_slice(),
            Dir::Left => self.left_data.as_mut_slice(),
            Dir::Right => self.right_data.as_mut_slice(),
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

pub fn a(input: &str) -> i32 {
    let mut map = Map::new(input);

    let start = map.find_first(b'S').unwrap();
    let end = map.find_first(b'E').unwrap();

    map.set(start, b'.');
    map.set(end, b'.');

    let mut has_visited = PosMap::<bool>::empty(map.width, map.height, false);

    let start_cost = manhattan(start, end);
    let start = Pos {
        pos: start,
        dir: Dir::Right,
    };

    let mut g_score = PosMap::<u32>::empty(map.width, map.height, u32::MAX);
    g_score.set(start, 0);

    let mut f_score = PosMap::<u32>::empty(map.width, map.height, u32::MAX);
    f_score.set(start, start_cost);

    let mut open_set = BinaryHeap::new();
    open_set.push(Cost {
        pos: start,
        cost: start_cost,
    });

    while let Some(Cost { pos: current, .. }) = open_set.pop() {
        if has_visited.get(current) {
            continue;
        }

        has_visited.set(current, true);

        if current.pos == end {
            return g_score.get(current) as i32;
        }

        for (neighbor, step_cost) in [
            (Pos::new(current.pos + current.dir.fwd(), current.dir), 1),
            (Pos::new(current.pos, current.dir.turn_left()), 1000),
            (Pos::new(current.pos, current.dir.turn_right()), 1000),
        ] {
            if map.get(neighbor.pos) == b'#' {
                continue;
            }

            let tentative_g_score = g_score.get(current) + step_cost;
            if tentative_g_score < g_score.get(neighbor) {
                let neighbor_f_score = tentative_g_score + manhattan(neighbor.pos, end);

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
    assert_eq!(a(TEST_INPUT), 7036);
    assert_eq!(a(TEST_INPUT_2), 11048);
    assert_eq!(a(INPUT), 94436);
}

struct CameFrom {
    up_data: Vec<Vec<Pos>>,
    down_data: Vec<Vec<Pos>>,
    left_data: Vec<Vec<Pos>>,
    right_data: Vec<Vec<Pos>>,
    width: i32,
    height: i32,
}

impl CameFrom {
    fn empty(width: i32, height: i32) -> CameFrom {
        CameFrom {
            up_data: vec![Vec::with_capacity(2); (width * height) as usize],
            down_data: vec![Vec::with_capacity(2); (width * height) as usize],
            left_data: vec![Vec::with_capacity(2); (width * height) as usize],
            right_data: vec![Vec::with_capacity(2); (width * height) as usize],
            width,
            height,
        }
    }

    fn get_mut(&mut self, pos: Pos) -> &mut Vec<Pos> {
        let data = match pos.dir {
            Dir::Up => self.up_data.as_mut_slice(),
            Dir::Down => self.down_data.as_mut_slice(),
            Dir::Left => self.left_data.as_mut_slice(),
            Dir::Right => self.right_data.as_mut_slice(),
        };

        let pos = pos.pos;

        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            panic!();
        }

        if pos.y < 0 || pos.y >= self.height {
            panic!();
        }

        &mut data[index as usize]
    }
}

fn count_path(mut map: Map<u8>, came_from: &mut CameFrom, current: Pos, start: Pos) -> i32 {
    let mut steps = 0;

    let mut open_set = vec![current];

    while let Some(current) = open_set.pop() {
        if map.get(current.pos) == b'.' {
            steps += 1;
        }
        map.set(current.pos, b'O');

        if current != start {
            let came_from = came_from.get_mut(current);
            for new in came_from {
                open_set.push(*new);
            }
        }
    }

    steps
}

pub fn b(input: &str) -> i32 {
    let mut map = Map::new(input);

    let start = map.find_first(b'S').unwrap();
    let end = map.find_first(b'E').unwrap();

    map.set(start, b'.');
    map.set(end, b'.');

    let mut has_visited = PosMap::<bool>::empty(map.width, map.height, false);

    let start_cost = manhattan(start, end);
    let start = Pos {
        pos: start,
        dir: Dir::Right,
    };
    let mut g_score = PosMap::<u32>::empty(map.width, map.height, u32::MAX);
    g_score.set(start, 0);

    let mut f_score = PosMap::<u32>::empty(map.width, map.height, u32::MAX);
    f_score.set(start, start_cost);

    let mut open_set = BinaryHeap::new();
    open_set.push(Cost {
        pos: start,
        cost: start_cost,
    });

    let mut came_from = CameFrom::empty(map.width, map.width);

    while let Some(Cost { pos: current, .. }) = open_set.pop() {
        if has_visited.get(current) {
            continue;
        }

        has_visited.set(current, true);

        if current.pos == end {
            return count_path(map.clone(), &mut came_from, current, start);
        }

        for (neighbor, step_cost) in [
            (Pos::new(current.pos + current.dir.fwd(), current.dir), 1),
            (Pos::new(current.pos, current.dir.turn_left()), 1000),
            (Pos::new(current.pos, current.dir.turn_right()), 1000),
        ] {
            if map.get(neighbor.pos) == b'#' {
                continue;
            }

            let tentative_g_score = g_score.get(current) + step_cost;
            let neighbor_g_score = g_score.get(neighbor);
            if tentative_g_score <= neighbor_g_score {
                let came_from = came_from.get_mut(neighbor);
                if tentative_g_score < neighbor_g_score {
                    came_from.clear();
                }
                came_from.push(current);

                let neighbor_f_score = tentative_g_score + manhattan(neighbor.pos, end);

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
fn test_b() {
    assert_eq!(b(TEST_INPUT), 45);
    assert_eq!(b(TEST_INPUT_2), 64);
    assert_eq!(b(INPUT), 481);
}
