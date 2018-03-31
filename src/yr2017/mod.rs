//! Year 2017 modules

mod processor;

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
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

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
        ("13-1", day13::star1),
        ("13-2", day13::star2),
        ("14-1", day14::star1),
        ("14-2", day14::star2),
        ("15-1", day15::star1),
        ("15-2", day15::star2),
        ("16-1", day16::star1),
        ("16-2", day16::star2),
        ("17-1", day17::star1),
        ("17-2", day17::star2),
        ("18-1", day18::star1),
        ("18-2", day18::star2),
        ("19-1", day19::star1),
        ("19-2", day19::star2),
        ("20-1", day20::star1),
        ("20-2", day20::star2),
        ("21-1", day21::star1),
        ("21-2", day21::star2),
        ("22-1", day22::star1),
        ("22-2", day22::star2),
        ("23-1", day23::star1),
        ("23-2", day23::star2),
        ("24-1", day24::star1),
        ("24-2", day24::star2),
    ]
}
