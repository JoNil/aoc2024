pub static INPUT: &str = include_str!("../input/7.txt");
pub static TEST_INPUT: &str = include_str!("../input/7_test.txt");

fn is_solvable(answer: i64, result: i64, remaining_numbers: &[i64]) -> bool {
    if remaining_numbers.is_empty() {
        return result == answer;
    }

    let new_result = result + remaining_numbers[0];
    let new_result = is_solvable(answer, new_result, &remaining_numbers[1..]);

    if new_result {
        return true;
    }

    let new_result = result * remaining_numbers[0];
    let new_result = is_solvable(answer, new_result, &remaining_numbers[1..]);

    if new_result {
        return true;
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
        if is_solvable(answer, 0, &numbers) {
            total_calibration_result += answer;
        }
    }

    total_calibration_result
}

fn count_digits(mut n: i64) -> i64 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;

    while n != 0 {
        n /= 10;
        count += 1;
    }

    count
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 3749);
    assert_eq!(a(INPUT), 5512534574980);
}

fn is_solvable_b(answer: i64, result: i64, remaining_numbers: &[i64]) -> bool {
    if remaining_numbers.is_empty() {
        return result == answer;
    }

    let new_result = result + remaining_numbers[0];
    let new_result = is_solvable_b(answer, new_result, &remaining_numbers[1..]);

    if new_result {
        return true;
    }

    let new_result = result * remaining_numbers[0];
    let new_result = is_solvable_b(answer, new_result, &remaining_numbers[1..]);

    if new_result {
        return true;
    }

    let digits = count_digits(remaining_numbers[0]);

    let new_result = result * 10i64.pow(digits as _) + remaining_numbers[0];
    let new_result = is_solvable_b(answer, new_result, &remaining_numbers[1..]);

    if new_result {
        return true;
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
        if is_solvable_b(answer, 0, &numbers) {
            total_calibration_result += answer;
        }
    }

    total_calibration_result
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 11387);
    assert_eq!(b(INPUT), 328790210468594);
}
