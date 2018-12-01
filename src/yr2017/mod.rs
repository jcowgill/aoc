//! Year 2017 modules

mod processor;
mod_all_days!();

use StarFunction; use StarId;
pub fn stars() -> Vec<(StarId, StarFunction)> { vec_all_days!(2017) }
