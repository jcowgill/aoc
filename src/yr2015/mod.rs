//! Year 2015 modules

mod day1;

use StarVector;

/// Returns the list of stars implemented this year
pub fn stars() -> StarVector {
    vec![
        ("01-1", day1::star1),
        ("01-2", day1::star2),
    ]
}
