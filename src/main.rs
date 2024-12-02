use std::time::Instant;

use aoc2022::day02;

fn main() {
    let start = Instant::now();
    assert!(day02::b(day02::INPUT) == 296);
    let elapsed = start.elapsed();

    println!("{}", elapsed.as_micros());
}
