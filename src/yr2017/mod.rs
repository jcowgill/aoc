//! Year 2017 modules

mod day1;

use aoclib::StarVector;

/// Returns the list of stars implemented this year
pub fn stars() -> StarVector {
    vec![
        ("01-1", day1::run1),
        ("01-2", day1::run2),
    ]
}
