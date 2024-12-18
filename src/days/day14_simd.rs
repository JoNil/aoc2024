use glam::IVec2;
use std::arch::x86_64::{
    _mm256_loadu_si256, _mm256_storeu_epi8, _mm512_add_epi16, _mm512_cmpge_epi16_mask,
    _mm512_cmplt_epi16_mask, _mm512_cvtepi16_epi8, _mm512_cvtepi8_epi16, _mm512_mask_add_epi16,
    _mm512_mask_sub_epi16, _mm512_set1_epi16,
};

#[derive(Clone)]
struct Map {
    data: Vec<i8>,
}

impl Map {
    fn empty(width: i32, height: i32) -> Map {
        Map {
            data: vec![0; (width * height) as usize],
        }
    }
}

#[derive(Default, Debug)]
struct Robots {
    pos_x: Vec<i8>,
    pos_y: Vec<i8>,
    speed_x: Vec<i8>,
    speed_y: Vec<i8>,
}

pub fn b(input: &str, size: IVec2) -> i32 {
    if cfg!(target_arch = "x86_64")
        && is_x86_feature_detected!("avx512f")
        && is_x86_feature_detected!("avx512bw")
        && is_x86_feature_detected!("avx2")
    {
        unsafe { b_avx_512(input, size) }
    } else {
        crate::day14::b(input, size)
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512bw,avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn b_avx_512(input: &str, size: IVec2) -> i32 {
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

        count += 1;
    }

    while robots.pos_x.len() < 512 {
        robots.pos_x.push(0);
        robots.pos_y.push(0);
        robots.speed_x.push(0);
        robots.speed_y.push(0);
    }

    let mut step = 0;

    let zero = _mm512_set1_epi16(0);
    let width = _mm512_set1_epi16(size.x as _);
    let height = _mm512_set1_epi16(size.y as _);

    loop {
        step += 1;

        let mut conflict = false;
        map.data.fill(0);

        const LANES: usize = 32;

        for i in (0..robots.pos_x.len()).step_by(LANES) {
            let x_addr = robots.pos_x.as_ptr().add(i);
            let y_addr = robots.pos_y.as_ptr().add(i);
            let dx_addr = robots.speed_x.as_ptr().add(i);
            let dy_addr = robots.speed_y.as_ptr().add(i);

            let x = _mm512_cvtepi8_epi16(_mm256_loadu_si256(x_addr as _));
            let y = _mm512_cvtepi8_epi16(_mm256_loadu_si256(y_addr as _));

            let dx = _mm512_cvtepi8_epi16(_mm256_loadu_si256(dx_addr as _));
            let dy = _mm512_cvtepi8_epi16(_mm256_loadu_si256(dy_addr as _));

            let mut new_x = _mm512_add_epi16(x, dx);
            let mut new_y = _mm512_add_epi16(y, dy);

            let mut new_x_wrapped_mask = _mm512_cmpge_epi16_mask(new_x, width);
            let mut new_y_wrapped_mask = _mm512_cmpge_epi16_mask(new_y, height);

            new_x = _mm512_mask_sub_epi16(new_x, new_x_wrapped_mask, new_x, width);
            new_y = _mm512_mask_sub_epi16(new_y, new_y_wrapped_mask, new_y, height);

            new_x_wrapped_mask = _mm512_cmplt_epi16_mask(new_x, zero);
            new_y_wrapped_mask = _mm512_cmplt_epi16_mask(new_y, zero);

            new_x = _mm512_mask_add_epi16(new_x, new_x_wrapped_mask, new_x, width);
            new_y = _mm512_mask_add_epi16(new_y, new_y_wrapped_mask, new_y, height);

            _mm256_storeu_epi8(x_addr as _, _mm512_cvtepi16_epi8(new_x));
            _mm256_storeu_epi8(y_addr as _, _mm512_cvtepi16_epi8(new_y));

            for ii in 0..LANES {
                let i = i + ii;
                if i >= count {
                    break;
                }

                let new_x = robots.pos_x[i];
                let new_y = robots.pos_y[i];

                let index = new_x as i32 + new_y as i32 * size.x;

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

    step
}

#[test]
fn test_b() {
    assert_eq!(b(crate::day14::INPUT, glam::ivec2(101, 103)), 7858);
}
