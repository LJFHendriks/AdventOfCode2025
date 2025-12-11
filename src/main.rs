use std::fs::read_to_string;
use std::time::Instant;

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        $(
        let instant = Instant::now();
        use aoc::$year::$day;
        let year = stringify!($year);
        let day = stringify!($day);
        let path = format!("input/{year}/{day}.txt");
        let input = &read_to_string(&path).expect(&format!("File not found at {}", path));
        let result = $day::parse(input);
        println!("{year} {day}");
        println!("PART 1: {}", $day::part1(&result).to_string());
        println!("PART 2: {}", $day::part2(&result).to_string());
        let elapsed = instant.elapsed();
        println!("🕓 {} ms", elapsed.as_millis());
        )*
    }
}

fn main()
{
    run!(year2025 day01, day02, day03, day04, day05, day06, day07, day08);
}
