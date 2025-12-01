use std::fs;

pub fn part1() {
    let mut sum = 0;
    let mut position = 50;
    let contents = fs::read_to_string("src/day01/input.txt").expect("File not found");
    for line in contents.split("\n") {
        let direction = &line[0..1];
        let amount: &i32 = &line[1..].trim().parse().expect("Was not able to parse to int");
        match direction {
            "L" => position -= amount,
            "R" => position += amount,
            _ => panic!("Unknown direction: {}", direction),
        }
        position = (position + 100) % 100;
        if position == 0 {
            sum += 1;
        }
    }
    println!("PART 1: {}", sum);
}

pub fn part2() {
    let mut sum = 0;
    let mut prev_position;
    let mut position = 50;
    let contents = fs::read_to_string("src/day01/input.txt").expect("File not found");
    for line in contents.split("\n") {
        prev_position = position;
        let direction = &line[0..1];
        let amount: &i32 = &line[1..].trim().parse().expect("Was not able to parse to int");
        match direction {
            "L" => position -= amount,
            "R" => position += amount,
            _ => panic!("Unknown direction: {}", direction),
        }
        if position >= 100 {
            sum += position / 100;
        }
        else if 0 >= position {
            sum += - position / 100;
            if prev_position != 0 {
                sum += 1;
            }
        }
        position = (position + 100000) % 100;
    }
    println!("PART 2: {}", sum);
}