use std::collections::{HashMap, HashSet, VecDeque};

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    // Queue to do breadth first search for the beam
    let mut beam_queue = VecDeque::new();
    // Hashmap to track the power of the beam at each position
    let mut beam_power = HashMap::new();
    // HashSet to track positions of splitters
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();
    // Maximum i to stop iteration
    let mut max_i = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.bytes().enumerate() {
            match char {
                b'S' => {
                    beam_queue.push_back((i, j));
                    beam_power.insert((i, j), 1);
                },
                b'^' => {splitters.insert((i, j)); },
                _ => ()
            };
        }
        if i > max_i {
            max_i = i;
        }
    }

    // Track splitters visited to count number of splits for part1
    let mut splitters_visited = HashSet::new();
    let mut part1 = 0;
    let mut part2 = 0;
    while let Some((i, j)) = beam_queue.pop_front() {
        let power = beam_power[&(i, j)];
        let (ni, nj) = (i + 1, j);
        if ni > max_i {
            // Add beam power to part2 when reached end
            part2 += power;
            continue;
        }
        // Spit beam if encountered splitter
        if splitters.contains(&(ni, nj)) {
            if splitters_visited.insert((ni, nj)) { part1 += 1 };
            insert_beam(ni, nj - 1, power, &mut beam_queue, &mut beam_power);
            insert_beam(ni, nj + 1, power, &mut beam_queue, &mut beam_power);
        }
        // else continue beam
        else {
            insert_beam(ni, nj, power, &mut beam_queue, &mut beam_power);
        }
    }
    (part1, part2)
}

fn insert_beam(i: usize, j: usize, power: usize, beam_queue: &mut VecDeque<(usize, usize)>, beam_power: &mut HashMap<(usize, usize), usize>) {
    // Check if the beam already exists
    *beam_power.entry((i, j)).or_insert_with(|| {
        // If the beam does not exist already we add it to the beam_queue to be checked and set default power to 0
        beam_queue.push_back((i, j));
        0
    // Add the power to the beam
    }) += power;
}

pub fn part1(input: &Input) -> usize {
    input.0
}

pub fn part2(input: &Input) -> usize {
    input.1
}