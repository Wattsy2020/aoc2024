fn parse_line(line: &str) -> Option<(i32, i32)> {
    line.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<i32>>()
        .split_at_checked(2)
        .map(|(first, _)| (first[0], first[1]))
}

fn solve(contents: &str) -> i32 {
    let (mut first_nums, mut second_nums): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .map(parse_line)
        .collect::<Option<Vec<(i32, i32)>>>()
        .expect("Failed to parse the file")
        .into_iter()
        .unzip();

    first_nums.sort();
    second_nums.sort();

    first_nums
        .iter()
        .zip(second_nums.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn solution() -> i32 {
    let contents =
        std::fs::read_to_string("days/day1.txt").expect("Should have been able to read the file");
    solve(&contents)
}
