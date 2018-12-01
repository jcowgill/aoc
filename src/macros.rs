//! Helper macros to avoid too much boilerplate

/// Import all days in a year (1 to 25)
macro_rules! mod_all_days
{
    () => {
        mod day1;  mod day2;  mod day3;  mod day4;  mod day5;
        mod day6;  mod day7;  mod day8;  mod day9;  mod day10;
        mod day11; mod day12; mod day13; mod day14; mod day15;
        mod day16; mod day17; mod day18; mod day19; mod day20;
        mod day21; mod day22; mod day23; mod day24; mod day25;
    };
}

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
macro_rules! vec_all_days
{
    ($year:expr) => {
        {
            let mut result: Vec<(StarId, StarFunction)> = vec_24_days!($year);
            result.push((StarId { year: $year, day: 25, star: 1 }, day25::star1));
            result
        }
    }
}
