use std::fs;

pub fn part1() {
    println!("PART 1");
    let mut position = 50;
    let contents = fs::read_to_string("src/day01/test_input.txt").expect("File not found");
    for line in contents.split("\n") {
        println!("{}", line);
        let direction = &line[0..1];
        let amount: &i32 = &line[1..].trim().parse().expect("Was not able to parse to int");
        match direction {
            "L" => position -= amount,
            "R" => position += amount,
            _ => panic!("Unknown direction: {}", direction),
        }

    }
}

pub fn part2() {
    println!("PART 2");
}