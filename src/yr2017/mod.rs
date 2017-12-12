//! Year 2017 modules

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

use StarVector;

/// Returns the list of stars implemented this year
pub fn stars() -> StarVector {
    vec![
        ("01-1", day1::star1),
        ("01-2", day1::star2),
        ("02-1", day2::star1),
        ("02-2", day2::star2),
        ("03-1", day3::star1),
        ("03-2", day3::star2),
        ("04-1", day4::star1),
        ("04-2", day4::star2),
        ("05-1", day5::star1),
        ("05-2", day5::star2),
        ("06-1", day6::star1),
        ("06-2", day6::star2),
        ("07-1", day7::star1),
        ("07-2", day7::star2),
        ("08-1", day8::star1),
        ("08-2", day8::star2),
        ("09-1", day9::star1),
        ("09-2", day9::star2),
        ("10-1", day10::star1),
        ("10-2", day10::star2),
        ("11-1", day11::star1),
        ("11-2", day11::star2),
        ("12-1", day12::star1),
        ("12-2", day12::star2),
    ]
}
