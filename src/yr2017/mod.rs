//! Year 2017 modules

mod day1;
mod day2;
mod day3;

use StarVector;

/// Returns the list of stars implemented this year
pub fn stars() -> StarVector {
    vec![
        ("01-1", day1::star1),
        ("01-2", day1::star2),
        ("02-1", day2::star1),
        ("02-2", day2::star2),
        ("03-1", day3::star1),
    ]
}
