use std::time::Instant;

pub static INPUT: &str = include_str!("../input/7.txt");
pub static TEST_INPUT: &str = include_str!("../input/7_test.txt");

fn is_solvable(answer: i64, result: i64, remaining_numbers: &[i64]) -> bool {
    if remaining_numbers.is_empty() {
        return result == answer;
    }

    let new_result = result + remaining_numbers[0];
    if new_result <= answer {
        let is_answer = is_solvable(answer, new_result, &remaining_numbers[1..]);

        if is_answer {
            return true;
        }
    }

    let new_result = result * remaining_numbers[0];
    if new_result <= answer {
        let is_answer = is_solvable(answer, new_result, &remaining_numbers[1..]);

        if is_answer {
            return true;
        }
    }

    false
}

pub fn a(input: &str) -> i64 {
    let mut equations = Vec::new();

    for line in input.lines() {
        let (answer, numbers) = line.split_once(':').unwrap();
        let answer = answer.parse::<i64>().unwrap();
        let numbers = numbers
            .trim()
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        equations.push((answer, numbers));
    }

    let mut total_calibration_result = 0;

    for (answer, numbers) in equations {
        if is_solvable(answer, numbers[0], &numbers[1..]) {
            total_calibration_result += answer;
        }
    }

    total_calibration_result
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3749);
    assert_eq!(a(INPUT), 5512534574980);
}

fn times_10_n(a: i64, n: i64) -> i64 {
    if n < 10 {
        a * 10
    } else if n < 100 {
        a * 100
    } else {
        a * 1000
    }
}

fn is_solvable_b(answer: i64, result: i64, remaining_numbers: &[i64], ops: &mut String) -> bool {
    if remaining_numbers.is_empty() {
        return result == answer;
    }

    let number = remaining_numbers[0];
    let rest = &remaining_numbers[1..];

    let new_result = result + number;
    if new_result <= answer {
        let is_answer = is_solvable_b(answer, new_result, rest, ops);

        if is_answer {
            *ops = format!("+{ops}");
            return true;
        }
    }

    let new_result = result * number;
    if new_result <= answer {
        let is_answer = is_solvable_b(answer, new_result, rest, ops);

        if is_answer {
            *ops = format!("*{ops}");
            return true;
        }
    }

    let new_result = times_10_n(result, number) + number;

    if new_result <= answer {
        let is_answer = is_solvable_b(answer, new_result, rest, ops);

        if is_answer {
            *ops = format!("|{ops}");
            return true;
        }
    }

    false
}

pub fn b(input: &str) -> i64 {
    let mut equations = Vec::new();

    for line in input.lines() {
        let (answer, numbers) = line.split_once(':').unwrap();
        let answer = answer.parse::<i64>().unwrap();
        let numbers = numbers
            .trim()
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        equations.push((answer, numbers));
    }

    let mut total_calibration_result = 0;

    for (answer, numbers) in equations {
        let start = Instant::now();

        let mut ops = String::new();

        let solved = if is_solvable_b(answer, numbers[0], &numbers[1..], &mut ops) {
            total_calibration_result += answer;
            true
        } else {
            false
        };

        let time = start.elapsed().as_micros();

        if time > 50 {
            print!("{}: ", answer);
            if solved {
                print!("{}", numbers[0]);
                for (n, op) in numbers[1..].iter().zip(ops.chars()) {
                    print!(" {op} {n}");
                }
            } else {
                for n in numbers {
                    print!("{n} ");
                }
            }

            println!(" time {}us", time,);
        }
    }

    total_calibration_result
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 11387);
    assert_eq!(b(INPUT), 328790210468594);
}
