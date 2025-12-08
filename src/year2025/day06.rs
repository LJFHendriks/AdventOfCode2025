use std::iter::zip;

type Input = (u64, u64);

pub fn parse(input: &str) -> Input {
    let mut lines: Vec<&str> = input.lines().collect();
    // pop last row to get the line of operators
    let operators = lines.pop().unwrap().split_whitespace().collect();
    // For part1 we split by whitespace and transpose the whole matrix
    let matrix = lines.iter().map(
        |line| line.split_whitespace().map(
            |value| value.parse().unwrap()
        ).collect()
    ).collect();
    let part1 = apply_operators(&transpose(&matrix), &operators);

    // For part 2 we transpose the matrix of characters,
    // these are then converted to String and trimmed.
    // The empty strings are then used to split the matrix into the values for each equation
    let matrix = lines.iter().map(
        |line| line.chars().collect()
    ).collect();
    let matrix = transpose(&matrix).into_iter().map(
        |line| line.into_iter().collect::<String>().trim().to_owned()
    ).collect::<Vec<String>>().split(
        |column| column.is_empty()
    ).map(
        |line| line.iter().map(
            |value| value.parse().unwrap()
        ).collect()
    ).collect();

    let part2 = apply_operators(&matrix, &operators);

    (part1, part2)
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len()).map(
        |j| (0..matrix.len()).map(
            |i| matrix[i][j].clone()
        ).collect()
    ).collect()
}

fn apply_operators(matrix: &Vec<Vec<u64>>, operators: &Vec<&str>) -> u64 {
    zip(matrix, operators).map(
        |(values, &operator)| {
            match operator {
                "*" => {
                    values.iter().fold(1, |total, value| total * value)
                }
                "+" => {
                    values.iter().fold(0, |total, value| total + value)
                }
                _ => {
                    panic!("Unsupported operator: {}", operator);
                }
            }
        }
    ).sum()
}

pub fn part1(input: &Input) -> u64 {
    input.0
}

pub fn part2(input: &Input) -> u64 {
    input.1
}