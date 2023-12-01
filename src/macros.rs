//! Helper macros to avoid too much boilerplate

/// Generates a vector containing 48 stars (day 1 to 24)
macro_rules! vec_24_days
{
    (@real $year:expr, $($num:expr, $mod:ident),*) => {
        vec![$(
            (StarId { year: $year, day: $num, star: 1 }, $mod::star1),
            (StarId { year: $year, day: $num, star: 2 }, $mod::star2),
        )*]
    };

    ($year:expr) => { vec_24_days!(@real $year,
         1, day1,   2, day2,   3, day3,   4, day4,   5, day5,
         6, day6,   7, day7,   8, day8,   9, day9,  10, day10,
        11, day11, 12, day12, 13, day13, 14, day14, 15, day15,
        16, day16, 17, day17, 18, day18, 19, day19, 20, day20,
        21, day21, 22, day22, 23, day23, 24, day24
    )};
}

/// Generates a vector containing all 49 stars (minus 25-2) for the
/// current year. Designed to be returned from a "stars" function.
macro_rules! vec_all_days {
    ($year:expr) => {{
        let mut result: Vec<(StarId, StarFunction)> = vec_24_days!($year);
        result.push((
            StarId {
                year: $year,
                day: 25,
                star: 1,
            },
            day25::star1,
        ));
        result
    }};
}

/// Implement a single star test
#[cfg(test)]
macro_rules! star_test {
    ($name:ident, $func:expr, $input: expr, $output: expr) => {
        #[test]
        fn $name() {
            assert_eq!($func($input.trim_end()), $output.trim_end());
        }
    };
}
