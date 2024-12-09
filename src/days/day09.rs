pub static INPUT: &str = include_str!("../input/9.txt");
pub static TEST_INPUT: &str = include_str!("../input/9_test.txt");

struct Block {
    index: i32,
    file: i32,
    free: i32,
}

pub fn a(input: &str) -> i64 {
    let mut blocks = Vec::new();

    for (index, chunk) in input.trim().as_bytes().chunks(2).enumerate() {
        if chunk.len() == 2 {
            blocks.push(Block {
                index: index as i32,
                file: (chunk[0] - 48) as i32,
                free: (chunk[1] - 48) as i32,
            });
        } else {
            blocks.push(Block {
                index: index as i32,
                file: (chunk[0] - 48) as i32,
                free: 0,
            })
        }
    }

    let mut to_index = 0;
    let mut from_index = blocks.len() - 1;

    while to_index < from_index {
        if blocks[from_index].file == 0 {
            from_index -= 1;
            continue;
        }

        if blocks[to_index].free == 0 {
            to_index += 1;
            continue;
        }

        if blocks[to_index].index == blocks[from_index].index {
            blocks[to_index].file += 1;
            blocks[to_index].free -= 1;
            blocks[from_index].file -= 1;
            blocks[from_index].free += 1;
        } else {
            blocks.insert(
                to_index + 1,
                Block {
                    index: blocks[from_index].index,
                    file: 0,
                    free: blocks[to_index].free,
                },
            );

            blocks[to_index].free = 0;

            from_index += 1;
            to_index += 1;
        }
    }

    let mut checksum = 0;
    let mut block_counter = 0;

    for block in blocks.iter() {
        for i in 0..block.file {
            checksum += ((i + block_counter) * block.index) as i64;
        }

        block_counter += block.file;
    }

    checksum
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 1928);
    assert_eq!(a(INPUT), 6384282079460);
}

pub fn b(input: &str) -> i64 {
    let mut blocks = Vec::new();

    for (index, chunk) in input.trim().as_bytes().chunks(2).enumerate() {
        if chunk.len() == 2 {
            blocks.push(Block {
                index: index as i32,
                file: (chunk[0] - 48) as i32,
                free: (chunk[1] - 48) as i32,
            });
        } else {
            blocks.push(Block {
                index: index as i32,
                file: (chunk[0] - 48) as i32,
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
            checksum += ((i + block_counter) * block.index) as i64;
        }

        block_counter += block.file;
        block_counter += block.free;
    }

    checksum
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 2858);
    assert_eq!(b(INPUT), 6408966547049);
}
