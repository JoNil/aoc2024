use std::time::Instant;

use aoc2022::*;

fn main() {
    let start = Instant::now();
    day01::a(day01::INPUT);
    day01::b(day01::INPUT);

    day02::a(day02::INPUT);
    day02::b(day02::INPUT);

    day03::a(day03::INPUT);
    day03::b(day03::INPUT);

    day04::a(day04::INPUT);
    day04::b(day04::INPUT);

    day05::a(day05::INPUT);
    day05::b(day05::INPUT);

    let elapsed = start.elapsed();

    println!("{}", elapsed.as_micros());
}
