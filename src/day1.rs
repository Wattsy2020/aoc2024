use crate::core::Countable;

/// Parses a single line of space-separated numbers into a tuple of two integers
/// Returns None if parsing fails
fn parse_line(line: &str) -> Option<(i32, i32)> {
    line.split_whitespace()
        .map(|num| num.parse().ok())
        .collect::<Option<Vec<i32>>>()?
        .split_at_checked(2)
        .map(|(first, _)| (first[0], first[1]))
}

/// Parses the entire input file into two vectors:
/// - One containing all first numbers from each line
/// - One containing all second numbers from each line
fn parse_contents(contents: &str) -> (Vec<i32>, Vec<i32>) {
    contents
        .lines()
        .map(parse_line)
        .collect::<Option<Vec<(i32, i32)>>>()
        .expect("Failed to parse the file")
        .into_iter()
        .unzip()
}

#[allow(dead_code)]
fn solve_part1(contents: &str) -> usize {
    let (mut first_nums, mut second_nums) = parse_contents(contents);

    first_nums.sort();
    second_nums.sort();

    first_nums
        .iter()
        .zip(second_nums.iter())
        .map(|(a, b)| (a - b).abs() as usize)
        .sum()
}

fn solve_part2(contents: &str) -> usize {
    let (first_nums, second_nums) = parse_contents(contents);
    let counts = second_nums.into_iter().counts();
    first_nums
        .into_iter()
        .map(|num| num as usize * counts.get(&num).unwrap_or(&0))
        .sum()
}

pub fn solution() -> usize {
    let contents =
        std::fs::read_to_string("days/day1.txt").expect("Should have been able to read the file");
    solve_part2(&contents)
}
