use std::num::ParseIntError;
use crate::core::{Pair, Pairwise};

fn parse_line(line: &str) -> Result<Box<[i32]>, ParseIntError> {
    line.split(' ').map(|str| str.parse()).collect()
}

fn parse_contents(contents: &str) -> Result<Box<[Box<[i32]>]>, ParseIntError> {
    contents.split('\n').map(parse_line).collect()
}

fn is_valid_positive_diff(diff: i32) -> bool {
    diff > 0 && diff <= 3
}

fn is_valid_negative_diff(diff: i32) -> bool {
    diff < 0 && diff >= -3
}

fn is_valid_report(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let diffs: Box<[i32]> = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    diffs.iter().all(|diff| is_valid_positive_diff(*diff))
        || diffs.iter().all(|diff| is_valid_negative_diff(*diff))
}

struct DiffDirectionIndices {
    increasing: Vec<usize>,
    decreasing: Vec<usize>
}

impl DiffDirectionIndices {
    fn count_diff(&mut self, diff: i32, idx: usize) {
        if diff >= 0 {
            self.increasing.push(idx);
        } else {
            self.decreasing.push(idx)
        }
    }

    fn count_diffs(diffs: &[i32]) -> Self {
        diffs
            .into_iter()
            .enumerate()
            .fold(Self::new(), |mut indices, (idx, diff)| {
                indices.count_diff(*diff, idx);
                indices
            })
    }

    fn new() -> Self {
        DiffDirectionIndices {
            increasing: Vec::new(),
            decreasing: Vec::new()
        }
    }
}

fn validate_diffs<F: Fn(i32) -> bool>(diffs: &[i32], is_valid_diff: F) -> bool {
    let mut level_removed = false;
    let mut prev_level_removed = false;

    diffs
        .iter()
        .pairwise()
        .all(|diffs| {
            let temp_prev_level_removed = prev_level_removed;
            prev_level_removed = false;
            
            let result = match diffs {
                // if removing a number in the middle, e.g. removing b in [a b c]
                // we need to make sure the diff between a and c is valid
                Pair::Middle(diff1, diff2) => temp_prev_level_removed
                    || is_valid_diff(*diff1)
                    || (!level_removed && {
                    level_removed = true;
                    prev_level_removed = true;
                    is_valid_diff(diff1 + diff2)
                }),
                // can just remove the first or last number to remove an invalid diff
                // between first and second number or second last and last number
                Pair::First(diff1, diff2) => is_valid_diff(*diff1)
                    || {
                    level_removed = true;
                    // strictly better to remove the middle number than the first, so try that
                    if is_valid_diff(diff1 + diff2) {
                        prev_level_removed = true;
                    }
                    true
                },
                Pair::Last(diff1, diff2) => ((temp_prev_level_removed || is_valid_diff(*diff1)) && is_valid_diff(*diff2))
                    || (!level_removed
                    && (is_valid_diff(diff1 + diff2) // remove second last number
                    || is_valid_diff(*diff1) // remove last number, request second last diff to be valid
                )),
            };
            result
        })
}

fn is_valid_report_with_dampener(report: &[i32]) -> bool {
    if report.len() <= 2 {
        return true;
    }

    let diffs: Box<[i32]> = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    // we need to handle the case where the first diff has the wrong direction
    // loop through diffs, count how many are in either direction
    // then fix the one with the wrong direction
    // todo: shouldn't need to check directions, just check if either direction works
    let diff_directions = DiffDirectionIndices::count_diffs(&diffs);
    match (diff_directions.increasing.as_slice(), diff_directions.decreasing.as_slice()) {
        ([] | [_], _) => validate_diffs(&diffs, is_valid_negative_diff),
        (_, [] | [_]) => validate_diffs(&diffs, is_valid_positive_diff),
        _ => false // neither is off by at most one, so it's not fixable
    }
}

#[allow(dead_code)]
fn solve_part1(reports: Box<[Box<[i32]>]>) -> usize {
    reports
        .iter()
        .filter(|report| is_valid_report(report))
        .count()
}

fn solve_part2(reports: Box<[Box<[i32]>]>) -> usize {
    <Box<[Box<[i32]>]> as IntoIterator>::into_iter(reports)
        .filter(|report| is_valid_report_with_dampener(report))
        .count()
}

#[allow(dead_code)]
pub fn solution() -> usize {
    let contents =
        std::fs::read_to_string("days/day2.txt").expect("Should have been able to read the file");
    let parsed_contents = parse_contents(&contents).expect("Puzzle input should be valid");
    solve_part2(parsed_contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_increase() {
        let test_data = vec![1, 100, 2, 4];
        assert!(is_valid_report_with_dampener(&test_data));
    }

    #[test]
    fn test_large_decrease() {
        let test_data = vec![1, -100, 2, 4];
        assert!(is_valid_report_with_dampener(&test_data));
    }

    #[test]
    fn test_double_large_increase() {
        assert!(!is_valid_report_with_dampener(&vec![1, 10, 20]));
        assert!(!is_valid_report_with_dampener(&vec![1, 10, 20, 21]));
        assert!(!is_valid_report_with_dampener(&vec![1, 10, 20, 10]));
    }

    #[test]
    fn test_double_large_swing() {
        assert!(!is_valid_report_with_dampener(&vec![1, -10, -5]));
        assert!(!is_valid_report_with_dampener(&vec![1, -10, -5, 21]));
        assert!(!is_valid_report_with_dampener(&vec![1, -10, -5, 10]));
    }

    #[test]
    fn test_negative_start_removal() {
        // [-5, -1]
        assert!(is_valid_report_with_dampener(&vec![10, 5, 4, 3]));
        assert!(is_valid_report_with_dampener(&vec![10, 5, 6, 7]));
        assert!(!is_valid_report_with_dampener(&vec![10, 5, 5, 4]));
    }

    #[test]
    fn test_positive_start_removal() {
        assert!(is_valid_report_with_dampener(&vec![10, 14, 7, 5]));
        assert!(!is_valid_report_with_dampener(&vec![10, 14, 5, 4]));
        assert!(is_valid_report_with_dampener(&vec![0, 5, 4, 3]));
        assert!(is_valid_report_with_dampener(&vec![0, 5, 6, 7]));
        assert!(!is_valid_report_with_dampener(&vec![0, 5, 5, 6]));
    }

    #[test]
    fn test_edgecases() {
        assert!(!is_valid_report_with_dampener(&vec![86, 89, 90, 93, 93, 95, 94]))
    }
}