use std::{any::type_name, fmt::Display, time::Instant};

use aoc2024::*;
use glam::ivec2;

fn time<F, N>(f: F, input: &str)
where
    F: FnOnce(&str) -> N,
    N: Display,
{
    let start = Instant::now();
    let answer = f(input);
    let elapsed = start.elapsed();

    println!(
        "{} Time {} us: {answer}",
        type_name::<F>().trim_start_matches("aoc2024::days::"),
        elapsed.as_micros()
    );
}

fn main() {
    let start = Instant::now();

    time(day01::a, day01::INPUT);
    time(day01::b, day01::INPUT);

    time(day02::a, day02::INPUT);
    time(day02::b, day02::INPUT);

    time(day03::a, day03::INPUT);
    time(day03::b, day03::INPUT);

    time(day04::a, day04::INPUT);
    time(day04::b, day04::INPUT);

    time(day05::a, day05::INPUT);
    time(day05::b, day05::INPUT);

    time(day06::a, day06::INPUT);
    time(day06::b, day06::INPUT);

    time(day07::a, day07::INPUT);
    time(day07::b, day07::INPUT);

    time(day08::a, day08::INPUT);
    time(day08::b, day08::INPUT);

    time(day09::a, day09::INPUT);
    time(day09::b, day09::INPUT);

    time(day10::a, day10::INPUT);
    time(day10::b, day10::INPUT);

    time(day11::a, day11::INPUT);
    time(day11::b, day11::INPUT);

    time(day12::a, day12::INPUT);
    time(day12::b, day12::INPUT);

    time(day13::a, day13::INPUT);
    time(day13::b, day13::INPUT);

    time(|input| day14::a(input, ivec2(101, 103)), day14::INPUT);
    time(|input| day14_simd::b(input, ivec2(101, 103)), day14::INPUT);

    time(day15::a, day15::INPUT);
    time(day15::b, day15::INPUT);

    time(day16::a, day16::INPUT);
    time(day16::b, day16::INPUT);

    time(day17::a, day17::INPUT);
    time(day17::b, day17::INPUT);

    time(|input| day18::a(input, ivec2(71, 71), 1024), day18::INPUT);
    time(|input| day18::b(input, ivec2(71, 71)), day18::INPUT);

    time(day19::a, day19::INPUT);
    time(day19::b, day19::INPUT);

    time(|input| day20::a(input, 100), day20::INPUT);
    time(|input| day20::b(input, 100), day20::INPUT);

    //time(day21::a, day21::INPUT);
    //time(day21::b, day21::INPUT);

    time(day22::a, day22::INPUT);
    time(day22::b, day22::INPUT);

    let elapsed = start.elapsed();

    println!("Total Time {} us", elapsed.as_micros());
}
