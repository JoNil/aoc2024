use std::time::Instant;

use aoc2022::day05 as day;

fn main() {
    let start = Instant::now();
    let val = day::b(day::INPUT);
    let elapsed = start.elapsed();

    println!("{}", val);
    println!("{}", elapsed.as_micros());
}
