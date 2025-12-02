type Input = Vec<i32>;

pub fn parse(input: &str) -> Input {
    input.lines().map(|line| {
        let (dir, amount) = line.split_at(1);
        amount.parse::<i32>().unwrap() * if dir == "L" { -1 } else { 1 }
    }).collect()
}

pub fn part1(input: &Input) -> i32 {
    // 1177
    let mut sum = 0;
    let mut position = 50;
    for movement in input {
        position += movement;
        position = position.rem_euclid(100);
        sum += i32::from(position == 0);
    }
    sum
}

pub fn part2(input: &Input) -> i32 {
    // 6768
    let mut sum = 0;
    let mut position = 50;
    for &movement in input {
        if movement >= 0 {
            sum += (position + movement) / 100;
        }
        else {
            let reversed = (100 - position) % 100;
            sum += (reversed - movement) / 100;
        }
        position += movement;
        position = position.rem_euclid(100);
    }
    sum
}