type Input = Vec<Vec<u8>>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut sum = 0;
    for battery in input {
        sum += find_joltage(battery, 2);
    }
    sum
}

pub fn part2(input: &Input) -> u64 {
    let mut sum = 0;
    for battery in input {
        sum += find_joltage(battery, 12);
    }
    sum
}

fn find_joltage(battery: &Vec<u8>, n: usize) -> u64 {
    let mut result: u64 = 0;
    let mut index = 0;
    for digit in (0..n).rev() {
        if let Some((highest_index, &maximum_digit)) = battery[index..battery.len()-digit].iter().enumerate().max_by_key(|&(i, x)| (x, -(i as isize))) {
            result = result * 10 + maximum_digit as u64;
            index = index + highest_index + 1;
        }
    }
    result
}