type Input = Vec<(u64, u64)>;

pub fn parse(input: &str) -> Input {
    input.split(",").map(|line| {
        // Map line to correct input
        let range: Vec<&str> = line.split("-").collect();
        (range[0].parse().unwrap(), range[1].parse().unwrap())
    }).collect()
}

pub fn part1(input: &Input) -> u64 {
    let invalids = find_invalids(find_length_of_max(input));
    sum_over_ranges(&invalids, input)
}

pub fn part2(input: &Input) -> u64 {
    let invalids = find_invalids_part_2(find_length_of_max(input));
    sum_over_ranges(&invalids, input)
}

fn find_length_of_max(input: &Input) -> u32 {
    let mut max = 0;
    for (a, b) in input {
        if *a > max {
            max = *a;
        }
        if *b > max {
            max = *b;
        }
    }
    max.ilog10() + 1
}

fn find_invalids(max_length: u32) -> Vec<u64> {
    let mut invalids = Vec::new();
    for digits in 2..=max_length {
        let size = digits / 2;
        let start = 10_u64.pow(size - 1);
        let end = 10_u64.pow(size);
        let base = end + 1;
        invalids.extend((start..end).map(|value| value * base));
    }
    invalids
}

fn find_invalids_part_2(max_length: u32) -> Vec<u64> {
    let mut invalids = Vec::new();
    for digits in 2..=max_length {
        for size in 1..=digits/2 {
            let repeats = digits / size;
            if digits % size != 0 {
                continue;
            }
            let start = 10_u64.pow(size - 1);
            let end = 10_u64.pow(size);
            let base = (0..repeats).fold(0, |acc, _| acc * end + 1);
            invalids.extend((start..end).map(|value| value * base));
        }
    }
    invalids.sort_unstable();
    invalids.dedup();
    invalids
}

fn sum_over_ranges(invalids: &Vec<u64>, ranges: &Input) -> u64 {
    let index = |n| invalids.binary_search(&n).unwrap_or_else(|i| i);
    ranges.iter().flat_map(|&(start, end)| invalids[index(start)..index(end + 1)].iter()).sum()
}