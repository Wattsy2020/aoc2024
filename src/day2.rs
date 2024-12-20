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

fn skip_index<T>(slice: &[T], idx: usize) -> impl Iterator<Item = &T> {
    slice
        .iter()
        .take(idx)
        .chain(slice.iter().skip(idx + 1))
}

fn is_valid_report_with_dampener(report: &[i32]) -> bool {
    if report.len() <= 1 {
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
        ([], _) => {
            let mut level_removed = false;
            diffs
                .windows(2)
                .all(|diffs| is_valid_negative_diff(diffs[0])
                    || !level_removed && {
                    level_removed = true;
                    is_valid_negative_diff(diffs[0] + diffs[1])
                })
        }
        (_, []) => {
            let mut level_removed = false;
            diffs
                .windows(2)
                .all(|diffs| is_valid_positive_diff(diffs[0])
                    || !level_removed && {
                    level_removed = true;
                    is_valid_positive_diff(diffs[0] + diffs[1])
                })
        }
        ([wrong_idx], _) => {
            // we're skipping a diff when we need to skip a number instead
            skip_index(&diffs, *wrong_idx)
                .all(|diff| is_valid_negative_diff(*diff))
        }
        (_, [wrong_idx]) => {
            let skipped: Box<[i32]> = skip_index(&diffs, *wrong_idx).cloned().collect();
            skipped.iter()
                .all(|diff| is_valid_positive_diff(*diff))
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
    let test_data = vec![1, 100, 2, 4];
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
    fn test_skip_idx() {
        let result: Vec<i32> = skip_index(&vec![1, 2, 3], 0).cloned().collect();
        assert_eq!(result, vec![2, 3]);
        let result: Vec<i32> = skip_index(&vec![1, 2, 3], 1).cloned().collect();
        assert_eq!(result, vec![1, 3]);
        let result: Vec<i32> = skip_index(&vec![1, 2, 3], 2).cloned().collect();
        assert_eq!(result, vec![1, 2]);
    }

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
}