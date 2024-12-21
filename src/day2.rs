use std::num::ParseIntError;

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
    let diff_directions = DiffDirectionIndices::count_diffs(&diffs);
    match (diff_directions.increasing.as_slice(), diff_directions.decreasing.as_slice()) {
        ([] | [_], _) => {
            let mut level_removed = false;
            let mut prev_level_removed = false;

            diffs
                .windows(2)
                .all(|diffs| { let temp = prev_level_removed; prev_level_removed = false; temp }
                    || is_valid_negative_diff(diffs[0])
                    || !level_removed && {
                    level_removed = true;
                    prev_level_removed = true;
                    is_valid_negative_diff(diffs[0] + diffs[1])
                })
        }
        (_, [] | [_]) => {
            let mut level_removed = false;
            let mut prev_level_removed = false;
            let mut is_first = true;
            
            // todo bug: we don't consider the last diff
            // is there some way to check the first and last diff in a nicer manner?
            // what if we try adding padding at both ends
            // e.g. for the positive direction, we add a diff of 1 at the start and end of the list
            for diffs in diffs.windows(2) {
                if prev_level_removed {
                    prev_level_removed = false;
                    continue;
                }

                if is_valid_positive_diff(diffs[0]) {
                    is_first = true;
                    continue;
                }

                if !level_removed {
                    level_removed = true;
                    if is_valid_positive_diff(diffs[0] + diffs[1]) {
                        prev_level_removed = true;
                    } else if is_first {
                        println!("removing first element");
                    }
                    else {
                        return false;
                    }
                    is_first = false;
                    continue;
                }
                return false;
            }

            true
        }
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
    let good_reports: Vec<Box<[i32]>> =
        <Box<[Box<[i32]>]> as IntoIterator>::into_iter(reports)
        .filter(|report| is_valid_report_with_dampener(report))
        .collect();
    println!("{good_reports:#?}");
    good_reports.len()
}

#[allow(dead_code)]
pub fn solution() -> usize {
    // [1, 100, 2] fails because it decides the Vector is decreasing
    // have to let the diff checking change both the increasing and decreasing order perhaps
    // shouldn't apply to this problem though because it has size 5, solve the other bug instead
    let test_data = vec![1, 10, 20];
    let result = is_valid_report_with_dampener(&test_data);
    println!("{result}");

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
        assert!(is_valid_report_with_dampener(&vec![10, 5, 4]));
        assert!(is_valid_report_with_dampener(&vec![10, 5, 6]));
        assert!(!is_valid_report_with_dampener(&vec![10, 5, 5]));
    }

    #[test]
    fn test_positive_start_removal() {
        assert!(!is_valid_report_with_dampener(&vec![10, 14, 7]));
        assert!(is_valid_report_with_dampener(&vec![0, 5, 4]));
        assert!(is_valid_report_with_dampener(&vec![0, 5, 6]));
        assert!(!is_valid_report_with_dampener(&vec![0, 5, 5]));
    }
}