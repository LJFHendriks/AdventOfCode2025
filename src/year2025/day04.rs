use std::collections::VecDeque;

type Input = (u32, u32);

const ADJACENT: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn parse(input: &str) -> Input {
    let mut m: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            // Map line to correct input
            line.trim()
                .bytes()
                .map(|c| if c == b'@' { 1 } else { 0 })
                .collect()
        })
        .collect();

    let mut total = 0;
    let mut queue = VecDeque::new();

    for x in 0..m.len() {
        for y in 0..m[x].len() {
            total += check_position(x, y, &m, &mut queue);
        }
    }

    let part1 = total;

    while let Some((remove_or_check_value, x, y)) = queue.pop_front() {
        if remove_or_check_value {
            m[x][y] = 0;
        }
        else {
            total += check_position(x, y, &m, &mut queue);
        }
    }
    (part1, total)
}

fn check_position(
    x: usize,
    y: usize,
    m: &Vec<Vec<u8>>,
    queue: &mut VecDeque<(bool, usize, usize)>,
) -> u32 {
    if m[x][y] == 1
        && ADJACENT
            .into_iter()
            .filter(|&(dx, dy)| {
                m.get(x.wrapping_add_signed(dx))
                    .and_then(|row| row.get(y.wrapping_add_signed(dy)))
                    .is_some_and(|&value| value == 1)
            })
            .count()
            < 4
    {
        queue.push_front((true, x, y));
        queue.extend(
            ADJACENT
                .into_iter()
                .map(|(dx, dy)| (false, x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
                .filter(|&(_, x, y)| m.get(x).is_some_and(|row| row.get(y).is_some())),
        );
        return 1;
    }
    0
}

pub fn part1(input: &Input) -> u32 {
    input.0
}

pub fn part2(input: &Input) -> u32 {
    input.1
}
