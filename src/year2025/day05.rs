type Input = (Vec<(u64, u64)>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap_or_else(|| input.split_once("\r\n\r\n").unwrap());
    let fresh_ranges: Vec<(u64,u64)> = fresh_ranges.lines().map(
        |line| line.split_once("-").map(
            |(range1, range2)| (range1.parse().unwrap(), range2.parse::<u64>().unwrap() + 1)
        ).unwrap()
    ).collect();
    let ingredients: Vec<u64> = ingredients.lines().map(
        |line| line.parse().unwrap()
    ).collect();

    (fresh_ranges, ingredients)
}

pub fn part1((fresh_ranges, ingredients): &Input) -> u64 {
    let mut sum = 0;
    // Implement part 1
    for ingredient in ingredients {
        for (a, b) in fresh_ranges {
            if ingredient >= a && ingredient < b {
                sum += 1;
                break;
            }
        }
    }
    sum
}

pub fn part2((fresh_ranges, _): &Input) -> u64 {
    let mut sum= 0;
    // Implement part 2
    let mut ranges_to_check = fresh_ranges.clone();
    let mut checked_ranges: Vec<(u64, u64)> = Vec::new();
    while let Some((mut a, mut b)) = ranges_to_check.pop() {
        for &(ca, cb) in &checked_ranges {
            if a < ca {
                // a...ca...cb...b
                if b > cb {
                    ranges_to_check.push((cb, b));
                    b = ca;
                }
                // a...ca...b...cb
                else if b > ca {
                    b = ca
                }
            }
            else if a < cb {
                // ca...a...cb...b
                if b > cb {
                    a = cb;
                }
                // ca...a...b...cb
                else if b > ca {
                    b = a;
                    break
                }
            }
        }
        if b > a {
            sum += b - a;
            checked_ranges.push((a, b))
        }
    }
    sum
}