use glam::IVec2;
use std::arch::x86_64::{
    _mm256_add_epi16, _mm256_add_epi8, _mm256_blendv_epi8, _mm256_cmpgt_epi16, _mm256_cmpgt_epi8,
    _mm256_loadu_si256, _mm256_set1_epi16, _mm256_set1_epi8, _mm256_storeu_epi16,
    _mm256_storeu_epi8, _mm256_sub_epi16, _mm256_sub_epi8,
};

pub static INPUT: &str = include_str!("../input/14.txt");
pub static TEST_INPUT: &str = include_str!("../input/14_test.txt");

#[derive(Clone)]
struct Map {
    data: Vec<i8>,
    width: i8,
    height: i8,
}

impl Map {
    fn empty(width: i32, height: i32) -> Map {
        Map {
            data: vec![0; (width * height) as usize],
            width: width as i8,
            height: height as i8,
        }
    }

    fn modify(&mut self, x: i8, y: i8, delta: i8) -> i8 {
        if x < 0 || x >= self.width {
            return 0;
        }

        if y < 0 || y >= self.height {
            return 0;
        }

        let index = x as i32 + y as i32 * self.width as i32;

        let new = self.data[index as usize] + delta;
        self.data[index as usize] = new;

        new
    }
}

#[derive(Default, Debug)]
struct Robots {
    pos_x: Vec<i16>,
    pos_y: Vec<i16>,
    speed_x: Vec<i16>,
    speed_y: Vec<i16>,
}

pub fn a(input: &str, size: IVec2) -> i32 {
    let mut robots = Robots::default();

    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();

        let (x, y) = left.split_once(',').unwrap();
        let x = x.trim_start_matches("p=").parse().unwrap();
        let y = y.parse().unwrap();

        let (dx, dy) = right.split_once(',').unwrap();
        let dx = dx.trim_start_matches("v=").parse().unwrap();
        let dy = dy.parse().unwrap();

        robots.pos_x.push(x);
        robots.pos_y.push(y);
        robots.speed_x.push(dx);
        robots.speed_y.push(dy);
    }

    for _step in 0..100 {
        for i in 0..robots.pos_x.len() {
            robots.pos_x[i] = (robots.pos_x[i] + robots.speed_x[i]).rem_euclid(size.x as _);
            robots.pos_y[i] = (robots.pos_y[i] + robots.speed_y[i]).rem_euclid(size.y as _);
        }
    }

    let middle_x = size.x as i16 / 2;
    let middle_y = size.y as i16 / 2;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for i in 0..robots.pos_x.len() {
        let x = robots.pos_x[i];
        let y = robots.pos_y[i];

        if x < middle_x && y < middle_y {
            q1 += 1;
        } else if x > middle_x && y < middle_y {
            q2 += 1;
        } else if x < middle_x && y > middle_y {
            q3 += 1;
        } else if x > middle_x && y > middle_y {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT, glam::ivec2(11, 7)), 12);
    assert_eq!(a(INPUT, glam::ivec2(101, 103)), 221655456);
}

pub fn b(input: &str, size: IVec2) -> i32 {
    let mut robots = Robots::default();

    let mut map = Map::empty(size.x, size.y);

    let mut count = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();

        let (x, y) = left.split_once(',').unwrap();
        let x = x.trim_start_matches("p=").parse().unwrap();
        let y = y.parse().unwrap();

        let (dx, dy) = right.split_once(',').unwrap();
        let dx = dx.trim_start_matches("v=").parse().unwrap();
        let dy = dy.parse().unwrap();

        robots.pos_x.push(x);
        robots.pos_y.push(y);
        robots.speed_x.push(dx);
        robots.speed_y.push(dy);

        map.modify(x as i8, y as i8, 1);

        count += 1;
    }

    while robots.pos_x.len() < 512 {
        robots.pos_x.push(0);
        robots.pos_y.push(0);
        robots.speed_x.push(0);
        robots.speed_y.push(0);
    }

    let mut step = 0;

    unsafe {
        let zero = _mm256_set1_epi16(0);
        let width = _mm256_set1_epi16(size.x as _);
        let height = _mm256_set1_epi16(size.y as _);
        let width_sub1 = _mm256_set1_epi16((size.x - 1) as _);
        let height_sub1 = _mm256_set1_epi16((size.y - 1) as _);

        loop {
            step += 1;

            let mut conflict = false;
            map.data.fill(0);

            const LANES: usize = 16;

            for i in (0..robots.pos_x.len()).step_by(LANES) {
                let x_addr = robots.pos_x.as_ptr().add(i);
                let y_addr = robots.pos_y.as_ptr().add(i);
                let dx_addr = robots.speed_x.as_ptr().add(i);
                let dy_addr = robots.speed_y.as_ptr().add(i);

                let x = _mm256_loadu_si256(x_addr as _);
                let y = _mm256_loadu_si256(y_addr as _);

                let dx = _mm256_loadu_si256(dx_addr as _);
                let dy = _mm256_loadu_si256(dy_addr as _);

                let mut new_x = _mm256_add_epi16(x, dx);
                let mut new_y = _mm256_add_epi16(y, dy);

                let mut new_x_wrapped_mask = _mm256_cmpgt_epi16(new_x, width_sub1);
                let mut new_y_wrapped_mask = _mm256_cmpgt_epi16(new_y, height_sub1);

                let mut wrapped_x = _mm256_sub_epi16(new_x, width);
                let mut wrapped_y = _mm256_sub_epi16(new_y, height);

                new_x = _mm256_blendv_epi8(new_x, wrapped_x, new_x_wrapped_mask);
                new_y = _mm256_blendv_epi8(new_y, wrapped_y, new_y_wrapped_mask);

                new_x_wrapped_mask = _mm256_cmpgt_epi16(zero, new_x);
                new_y_wrapped_mask = _mm256_cmpgt_epi16(zero, new_y);

                wrapped_x = _mm256_add_epi16(new_x, width);
                wrapped_y = _mm256_add_epi16(new_y, height);

                new_x = _mm256_blendv_epi8(new_x, wrapped_x, new_x_wrapped_mask);
                new_y = _mm256_blendv_epi8(new_y, wrapped_y, new_y_wrapped_mask);

                _mm256_storeu_epi16(x_addr as _, new_x);
                _mm256_storeu_epi16(y_addr as _, new_y);

                for ii in 0..LANES {
                    let i = i + ii;
                    if i >= count {
                        break;
                    }

                    let new_x = robots.pos_x[i];
                    let new_y = robots.pos_y[i];

                    let index = new_x as i32 + new_y as i32 * map.width as i32;

                    let robots_in_pos = map.data[index as usize] + 1;
                    map.data[index as usize] = robots_in_pos;

                    if robots_in_pos > 1 {
                        conflict = true;
                    }
                }
            }

            if !conflict {
                break;
            }
        }
    }

    step
}

#[test]
fn test_b() {
    assert_eq!(b(INPUT, glam::ivec2(101, 103)), 7858);
}
