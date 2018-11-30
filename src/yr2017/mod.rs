//! Year 2017 modules

mod processor;
mod_all_days!();

use StarVector;

/// Returns the list of stars implemented this year
pub fn stars() -> StarVector {
    vec_all_days!()
}
