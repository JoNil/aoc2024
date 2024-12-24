use crate::{AdventHashMap, AdventHashSet};
use glam::{ivec2, IVec2};
use std::{fmt::Display, str};

pub static INPUT: &str = include_str!("../input/23.txt");
pub static TEST_INPUT: &str = include_str!("../input/23_test.txt");

pub fn a(input: &str) -> i32 {
    let mut computers = AdventHashMap::<&str, AdventHashSet<&str>>::default();

    for (a, b) in input.lines().map(|l| l.split_once('-').unwrap()) {
        computers.entry(a).or_default().insert(b);
        computers.entry(b).or_default().insert(a);
    }

    let mut unique_paths = AdventHashSet::default();

    for (start, next) in computers.iter().filter(|c| c.0.starts_with('t')) {
        for second in next {
            let second_next = computers.get(second).unwrap();
            for third in second_next {
                if next.contains(third) {
                    let mut id = [*start, *second, *third];
                    id.sort();

                    if !unique_paths.contains(&id) {
                        unique_paths.insert(id);
                        println!("{start} {second} {third}");
                    }
                }
            }
        }
    }

    unique_paths.len() as _
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 7);
    assert_eq!(a(INPUT), 1154);
}

#[derive(Clone)]
pub struct Matrix {
    data: Vec<i32>,
    width: i32,
    height: i32,
}

impl Matrix {
    pub fn empty(width: usize, height: usize) -> Matrix {
        Matrix {
            data: vec![0; width * height],
            width: width as i32,
            height: height as i32,
        }
    }

    pub fn get(&self, pos: IVec2) -> i32 {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return 0;
        }

        if pos.y < 0 || pos.y >= self.height {
            return 0;
        }

        unsafe { *self.data.get_unchecked(index as usize) }
    }

    pub fn set(&mut self, pos: IVec2, new: i32) -> bool {
        let index = pos.x + pos.y * self.width;

        if pos.x < 0 || pos.x >= self.width {
            return false;
        }

        if pos.y < 0 || pos.y >= self.height {
            return false;
        }

        unsafe {
            *self.data.get_unchecked_mut(index as usize) = new;
        }

        true
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.chunks(self.width as usize) {
            for c in line {
                write!(f, "{}", (*c + 48) as u8 as char)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn b(input: &str) -> i32 {
    let mut connections = AdventHashMap::<&str, AdventHashSet<&str>>::default();

    for (a, b) in input.lines().map(|l| l.split_once('-').unwrap()) {
        connections.entry(a).or_default().insert(b);
        connections.entry(b).or_default().insert(a);
    }

    let mut computers = connections.keys().copied().collect::<Vec<_>>();
    computers.sort();

    let reverse = computers
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<AdventHashMap<_, _>>();

    let mut matrix = Matrix::empty(computers.len(), computers.len());

    for (i, computer) in computers.iter().copied().enumerate() {
        for connection in connections.get(computer).unwrap().iter().copied() {
            let j = *reverse.get(connection).unwrap();

            matrix.set(ivec2(i as _, j as _), 1);
            matrix.set(ivec2(j as _, i as _), 1);
        }
    }

    for computers in computers.iter().copied() {
        print!("{}", computers.as_bytes()[0] as char)
    }

    println!();

    for computers in computers.iter().copied() {
        print!("{}", computers.as_bytes()[1] as char)
    }

    println!();

    println!("{}", matrix);

    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 1);
    assert_eq!(b(INPUT), 0);
}
