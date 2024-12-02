pub static INPUT: &str = include_str!("../input/2.txt");
pub static TEST_INPUT: &str = include_str!("../input/2_test.txt");

pub fn a(input: &str) -> i32 {
    let mut reports = Vec::new();

    for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        reports.push(levels);
    }

    let mut safe_reports = 0;

    for report in reports {
        let diffs = report[1..]
            .iter()
            .zip(report[..report.len() - 1].iter())
            .map(|(a, b)| *a - *b)
            .collect::<Vec<_>>();

        let all_signs_positive = diffs.iter().all(|d| d.signum() > 0);
        let all_signs_negative = diffs.iter().all(|d| d.signum() < 0);
        let all_diffs_less_then_3 = diffs.iter().all(|d| d.abs() <= 3);

        let safe = (all_signs_positive || all_signs_negative) && all_diffs_less_then_3;

        if safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

#[test]
fn test_a() {
    assert_eq!(a(TEST_INPUT), 2);
    assert_eq!(a(INPUT), 220);
}

pub fn b(input: &str) -> i32 {
    let mut reports = Vec::new();

    for line in input.lines() {
        let levels = line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        reports.push(levels);
    }

    let mut safe_reports = 0;

    for report in reports {
        for defect in 0..report.len() {
            let mut report = report.clone();
            report.remove(defect);

            let diffs = report[1..]
                .iter()
                .zip(report[..report.len() - 1].iter())
                .map(|(a, b)| *a - *b)
                .collect::<Vec<_>>();

            let all_signs_positive = diffs.iter().all(|d| d.signum() > 0);
            let all_signs_negative = diffs.iter().all(|d| d.signum() < 0);
            let all_diffs_less_then_3 = diffs.iter().all(|d| d.abs() <= 3);

            let safe = (all_signs_positive || all_signs_negative) && all_diffs_less_then_3;

            if safe {
                safe_reports += 1;
                break;
            }
        }
    }

    safe_reports
}

#[test]
fn test_b() {
    assert_eq!(b(TEST_INPUT), 4);
    assert_eq!(b(INPUT), 0);
}
