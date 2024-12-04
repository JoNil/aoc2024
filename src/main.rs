use std::time::Instant;

use aoc2022::day04 as day;

fn main() {
    let start = Instant::now();
    let val = day::a(day::INPUT);
    let elapsed = start.elapsed();

    println!("{}", val);
    println!("{}", elapsed.as_micros());
}
