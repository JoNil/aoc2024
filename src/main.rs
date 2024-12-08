use std::time::Instant;

use aoc2024::*;

fn main() {
    let start = Instant::now();
    //day01::a(day01::INPUT);
    //day01::b(day01::INPUT);

    //day02::a(day02::INPUT);
    //day02::b(day02::INPUT);

    //day03::a(day03::INPUT);
    //day03::b(day03::INPUT);

    //day04::a(day04::INPUT);
    //day04::b(day04::INPUT);

    //day05::a(day05::INPUT);
    //day05::b(day05::INPUT);

    //day06::a(day06::INPUT);
    //day06::b(day06::INPUT);

    //day07::a(day07::INPUT);
    //day07::b(day07::INPUT);

    //day08::a(day08::INPUT);
    day08::b(day08::INPUT);

    let elapsed = start.elapsed();

    println!("Time {} us", elapsed.as_micros());
}
