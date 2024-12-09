use std::iter;

pub static INPUT: &str = include_str!("../input/9.txt");
pub static TEST_INPUT: &str = include_str!("../input/9_test.txt");

struct Block {
    index: i32,
    file: i32,
    free: i32,
}

fn expand(blocks: &[Block]) {
    let mut res = String::new();

    for block in blocks.iter() {
        res.extend(iter::repeat((block.index + 48) as u8 as char).take(block.file as _));
        res.extend(iter::repeat('.').take(block.free as _));
    }

    println!("{}", res);
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

        //expand(&blocks);

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

    //expand(&blocks);

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

pub fn b(input: &str) -> i32 {
    0
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 0);
    assert_eq!(b(INPUT), 0);
}
