macro_rules! import {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

import!(util point3d);

import!(year2025 day01, day02, day03, day04, day05, day06, day07, day08);