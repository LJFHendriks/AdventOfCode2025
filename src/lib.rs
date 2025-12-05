macro_rules! import {
    ($year:tt $($day:tt),*) => {
        pub mod $year {
            $(pub mod $day;)*
        }
    }
}

import!(year2025 day01, day02, day03, day04, day05);