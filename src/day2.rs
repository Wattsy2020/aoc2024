use std::num::ParseIntError;

fn parse_line(line: &str) -> Result<Box<[i32]>, ParseIntError> {
    line.split(' ').map(|str| str.parse()).collect()
}

fn parse_contents(contents: &str) -> Result<Box<[Box<[i32]>]>, ParseIntError> {
    contents.split('\n').map(parse_line).collect()
}

fn is_valid_report(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let diffs: Box<[i32]> = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    diffs.iter().all(|diff| diff.abs() <= 3)
        && (diffs.iter().all(|diff| *diff > 0) || diffs.iter().all(|diff| *diff < 0))
}

fn solve_part1(reports: Box<[Box<[i32]>]>) -> usize {
    reports
        .iter()
        .filter(|report| is_valid_report(report))
        .count()
}

#[allow(dead_code)]
pub fn solution() -> usize {
    let contents =
        std::fs::read_to_string("days/day2.txt").expect("Should have been able to read the file");
    let parsed_contents = parse_contents(&contents).expect("Puzzle input should be valid");
    solve_part1(parsed_contents)
}
