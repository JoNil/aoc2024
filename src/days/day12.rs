use glam::{ivec2, IVec2};
use std::{collections::HashMap, iter};

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

#[derive(Debug)]
struct Region {
    min: IVec2,
    max: IVec2,
    positions: Vec<IVec2>,
}

impl Region {
    fn new(p: IVec2) -> Region {
        Region {
            min: p,
            max: p,
            positions: vec![p],
        }
    }

    fn touches(&self, p: IVec2) -> bool {
        let touches_bb = p.x >= self.min.x - 1
            && p.x <= self.max.x + 1
            && p.y >= self.min.y - 1
            && p.y <= self.max.y + 1;

        if touches_bb {
            for pos in &self.positions {
                let diff = (pos - p).abs();
                if diff == ivec2(1, 0) || diff == ivec2(0, 1) {
                    return true;
                }
            }
        }

        false
    }

    fn insert(&mut self, p: IVec2) {
        //assert!(self.touches(p));
        self.min = self.min.min(p);
        self.max = self.max.max(p);
        self.positions.push(p);
    }
}

#[test]
fn test_touches() {
    {
        let region = Region::new(ivec2(3, 0));
        assert!(region.touches(ivec2(2, 0)));
        assert!(region.touches(ivec2(4, 0)));

        assert!(!region.touches(ivec2(1, 0)));
        assert!(!region.touches(ivec2(5, 0)));
    }

    {
        let region = Region::new(ivec2(0, 3));
        assert!(region.touches(ivec2(0, 2)));
        assert!(region.touches(ivec2(0, 4)));

        assert!(!region.touches(ivec2(0, 1)));
        assert!(!region.touches(ivec2(0, 5)));
    }

    {
        let mut region = Region::new(ivec2(3, 2));
        region.insert(ivec2(2, 3));
        region.insert(ivec2(3, 3));

        assert!(region.touches(ivec2(2, 2)));
        assert!(region.touches(ivec2(4, 2)));

        assert!(!region.touches(ivec2(0, 1)));
        assert!(!region.touches(ivec2(0, 5)));
    }
}

pub fn a(input: &str) -> i32 {
    let map = Map::new(input);
    let mut regions = HashMap::<u8, Vec<Region>>::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let p = ivec2(x, y);
            let c = map.get(p);

            let regions = regions.entry(c).or_default();
            let mut touches = Vec::new();

            for (i, region) in regions.iter().enumerate() {
                if region.touches(p) {
                    touches.push(i);
                }
            }

            if touches.is_empty() {
                regions.push(Region::new(p));
            } else if touches.len() == 1 {
                regions[touches[0]].insert(p);
            } else {
                let mut main = None;
                let mut rest = Vec::new();
                let mut untouched = Vec::new();

                for (index, region) in regions.drain(..).enumerate() {
                    if touches.contains(&index) {
                        if main.is_none() {
                            main = Some(region);
                        } else {
                            rest.push(region);
                        }
                    } else {
                        untouched.push(region);
                    }
                }

                for region in rest {
                    for pos in region.positions {
                        main.as_mut().unwrap().insert(pos);
                    }
                }

                *regions = iter::once(main.unwrap())
                    .chain(untouched.drain(..))
                    .collect();

                for region in regions {
                    for p in &region.positions {
                        if !region.touches(*p) {
                            eprintln!("{}", c as char);
                            eprintln!("{region:?}");
                            panic!("Blä");
                        }
                    }
                }
            }
        }
    }

    let mut price = 0;

    for (label, regions) in &regions {
        for region in regions {
            let area = region.positions.len();
            let mut border = 0;

            for pos in &region.positions {
                for dir in [ivec2(1, 0), ivec2(-1, 0), ivec2(0, 1), ivec2(0, -1)] {
                    if map.get(pos + dir) != *label {
                        border += 1;
                    }
                }
            }

            price += area * border;

            println!("{} {area} * {border} = {}", *label as char, area * border);
        }
    }

    price as i32
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1930);
    //assert_eq!(a(INPUT), 0);
}

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
