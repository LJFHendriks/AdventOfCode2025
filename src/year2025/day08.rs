use std::collections::HashSet;
use crate::util::point3d::Point3D;

type Input = (usize, usize, usize);

pub fn parse(input: &str) -> Input {
    let points: Vec<Point3D<u32>> = input.lines().map(|line| {
        // Map line to correct input
        let parsed_line: Vec<_> = line.split(",").map(
            |value| value.parse().unwrap()
        ).collect();
        Point3D::new(parsed_line[0], parsed_line[1], parsed_line[2])
    }).collect();

    let mut distance_vec: Vec<(usize, usize, f64)> = Vec::new();
    for (i, point_a) in points.iter().enumerate() {
        for (j, point_b) in points.iter().enumerate() {
            if i >= j {
                continue;
            }
            distance_vec.push((i, j, point_a.distance(point_b)))
        }
    }

    distance_vec.sort_by(|(_, _, distance_a), (_, _, distance_b)| distance_a.total_cmp(distance_b));

    let mut connected: Vec<HashSet<usize>> = Vec::new();
    let mut part1_test = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, &(a, b, _)) in distance_vec.iter().enumerate() {
        if i == 10 {
            part1_test = calculate_score(&mut connected);
        }
        if i == 1000 {
            part1 = calculate_score(&mut connected);
        }
        let mut contained_in = Vec::new();
        for (i, subset) in connected.iter().enumerate() {
            if subset.contains(&a) {
                contained_in.push(i);
            }
            else if subset.contains(&b) {
                contained_in.push(i);
            }
        }
        match contained_in.len() {
            0 => {
                connected.push(HashSet::from([a, b]))
            }
            1 => {
                connected[contained_in[0]].extend([a, b]);
            }
            2 => {
                let temp = connected[contained_in[1]].clone();
                connected[contained_in[0]].extend(temp);
                connected.remove(contained_in[1]);
            }
            _ => {
                panic!("Too many connections");
            }
        }
        if connected.len() == 1 && connected[0].len() == points.len() {
            part2 = points[a].x * points[b].x;
            break;
        }
    }

    (part1_test, part1, part2 as usize)
}

fn calculate_score(connected: &mut Vec<HashSet<usize>>) -> usize {
    connected.sort_by_key(|a| {a.len()});

    connected[connected.len()-3..].iter().fold(1, |sum, sub_set| sum * sub_set.len())
}

pub fn part1_testable(input: &Input) -> usize {
    // Implement part 1
    input.0
}

pub fn part1(input: &Input) -> usize {
    input.1
}

pub fn part2(input: &Input) -> usize {
    input.2
}