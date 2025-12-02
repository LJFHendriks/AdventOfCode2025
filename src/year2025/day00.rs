type Input = Vec<i32>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        // Map line to correct input
        line.parse::<i32>().unwrap()
    }).collect()
}

pub fn part1(input: &Input) -> i32 {
    let mut sum = 0;
    // Implement part 1
    sum
}

pub fn part2(input: &Input) -> i32 {
    let mut sum = 0;
    // Implement part 2
    sum
}