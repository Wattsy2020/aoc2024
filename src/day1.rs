fn parse_line(line: &str) -> Option<(i32, i32)> {
    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    nums.get(0)
        .and_then(|first| nums.get(1).map(|second| (*first, *second)))
}

pub fn solution() -> i32 {
    let contents =
        std::fs::read_to_string("days/day1.txt").expect("Should have been able to read the file");

    let nums: Vec<(i32, i32)> = contents
        .lines()
        .map(parse_line)
        .collect::<Option<Vec<(i32, i32)>>>()
        .expect("Failed to parse the file");

    let mut first_num: Vec<i32> = Vec::new();
    let mut second_num: Vec<i32> = Vec::new();
    for (first, second) in nums {
        first_num.push(first);
        second_num.push(second);
    }

    first_num.sort();
    second_num.sort();

    first_num
        .iter()
        .zip(second_num.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}
