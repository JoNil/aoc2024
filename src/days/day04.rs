pub static INPUT: &str = include_str!("../input/4.txt");
pub static TEST_INPUT: &str = include_str!("../input/4_test.txt");

struct Map<'a> {
    data: &'a [u8],
    stride: i32,
}

impl Map<'_> {
    fn get(&self, x: i32, y: i32) -> u8 {
        let index = x + y * self.stride;

        if x < 0 || x >= self.stride {
            return b'.';
        }

        if y < 0 {
            return b'.';
        }

        if index >= self.data.len() as i32 || index < 0 {
            return b'.';
        }

        self.data[index as usize]
    }
}

pub fn a(input: &str) -> i32 {
    let mut map = Map {
        data: input.as_bytes(),
        stride: 0,
    };

    let mut lines: i32 = 0;

    for line in input.lines() {
        map.stride = line.len() as i32 + 1;
        lines += 1;
    }

    let mut count = 0;

    for y in 0..lines {
        for x in 0..map.stride {
            let first = map.get(x, y);

            if first == b'X' {
                let candiadtes_coords: [[(i32, i32); 3]; 4] = [
                    [(0, 1), (0, 2), (0, 3)],
                    [(1, 0), (2, 0), (3, 0)],
                    [(1, -1), (2, -2), (3, -3)],
                    [(1, 1), (2, 2), (3, 3)],
                ];

                for coords in candiadtes_coords {
                    let candidate = [
                        map.get(x + coords[0].0, y + coords[0].1),
                        map.get(x + coords[1].0, y + coords[1].1),
                        map.get(x + coords[2].0, y + coords[2].1),
                    ];

                    if &candidate[..] == b"MAS" {
                        count += 1;
                    }
                }
            }

            if first == b'S' {
                let candiadtes_coords: [[(i32, i32); 3]; 4] = [
                    [(0, 1), (0, 2), (0, 3)],
                    [(1, 0), (2, 0), (3, 0)],
                    [(1, -1), (2, -2), (3, -3)],
                    [(1, 1), (2, 2), (3, 3)],
                ];

                for coords in candiadtes_coords {
                    let candidate = [
                        map.get(x + coords[0].0, y + coords[0].1),
                        map.get(x + coords[1].0, y + coords[1].1),
                        map.get(x + coords[2].0, y + coords[2].1),
                    ];

                    if &candidate[..] == b"AMX" {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 18);
    assert_eq!(a(INPUT), 2500);
}

pub fn b(input: &str) -> i32 {
    let mut map = Map {
        data: input.as_bytes(),
        stride: 0,
    };

    let mut lines: i32 = 0;

    for line in input.lines() {
        map.stride = line.len() as i32 + 1;
        lines += 1;
    }

    let mut count = 0;

    for y in 0..lines {
        for x in 0..map.stride {
            let c1 = [(0, 2), (1, 1), (2, 0)];
            let c2 = [(0, 0), (1, 1), (2, 2)];

            let candidate_1 = [
                map.get(x + c1[0].0, y + c1[0].1),
                map.get(x + c1[1].0, y + c1[1].1),
                map.get(x + c1[2].0, y + c1[2].1),
            ];

            let candidate_2 = [
                map.get(x + c2[0].0, y + c2[0].1),
                map.get(x + c2[1].0, y + c2[1].1),
                map.get(x + c2[2].0, y + c2[2].1),
            ];

            if (&candidate_1[..] == b"MAS" || &candidate_1[..] == b"SAM")
                && (&candidate_2[..] == b"MAS" || &candidate_2[..] == b"SAM")
            {
                count += 1;
            }
        }
    }

    count
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 9);
    assert_eq!(b(INPUT), 1933);
}
