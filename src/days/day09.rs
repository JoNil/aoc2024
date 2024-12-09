use std::collections::VecDeque;

pub static INPUT: &str = include_str!("../input/9.txt");
pub static TEST_INPUT: &str = include_str!("../input/9_test.txt");

struct Block {
    index: i16,
    file: u8,
    free: u8,
}

pub fn a(input: &str) -> i64 {
    let mut blocks = Vec::with_capacity(input.len() / 2);

    for (index, chunk) in input.trim().as_bytes().chunks(2).enumerate() {
        if chunk.len() == 2 {
            blocks.push(Block {
                index: index as i16,
                file: chunk[0] - 48,
                free: chunk[1] - 48,
            });
        } else {
            blocks.push(Block {
                index: index as i16,
                file: chunk[0] - 48,
                free: 0,
            })
        }
    }

    let mut checksum = 0;
    let mut block_counter = 0;
    let mut free_index = blocks.len() - 1;

    for block_index in 0..blocks.len() {
        for i in 0..blocks[block_index].file {
            checksum += ((i as i32 + block_counter) * blocks[block_index].index as i32) as i64;
        }

        block_counter += blocks[block_index].file as i32;

        if free_index - 1 > block_index {
            for i in 0..blocks[block_index].free {
                if blocks[free_index].file == 0 {
                    free_index -= 1;
                }

                let index = blocks[free_index].index;
                blocks[free_index].file -= 1;

                checksum += ((i as i32 + block_counter) * index as i32) as i64;
            }

            block_counter += blocks[block_index].free as i32;
        }

        if blocks[block_index].file == 0 {
            break;
        }
    }

    checksum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1928);
    assert_eq!(a(INPUT), 6384282079460);
}

pub fn b(input: &str) -> i64 {
    let mut blocks = VecDeque::with_capacity(input.len());

    for (index, chunk) in input.trim().as_bytes().chunks(2).enumerate() {
        if chunk.len() == 2 {
            blocks.push_back(Block {
                index: index as i16,
                file: chunk[0] - 48,
                free: chunk[1] - 48,
            });
        } else {
            blocks.push_back(Block {
                index: index as i16,
                file: chunk[0] - 48,
                free: 0,
            })
        }
    }

    let mut from_index = blocks.len() - 1;
    let mut first_with_space = [0; 10];

    while from_index != 0 {
        let space = blocks[from_index].file;

        for to_index in first_with_space[space as usize]..from_index {
            if blocks[to_index].free >= space {
                blocks.insert(
                    to_index + 1,
                    Block {
                        index: blocks[from_index].index,
                        file: space,
                        free: blocks[to_index].free - space,
                    },
                );

                first_with_space[space as usize] = to_index;

                from_index += 1;

                blocks[to_index].free = 0;
                blocks[from_index].file = 0;
                blocks[from_index].free += space;

                break;
            }
        }

        from_index -= 1;
    }

    let mut checksum = 0;
    let mut block_counter = 0;

    for block in blocks.iter() {
        for i in 0..block.file {
            checksum += ((i as i32 + block_counter) * block.index as i32) as i64;
        }

        block_counter += block.file as i32;
        block_counter += block.free as i32;
    }

    checksum
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 2858);
    assert_eq!(b(INPUT), 6408966547049);
}
