//! Year 2015 modules

mod_all_days!();

use crate::StarFunction; use crate::StarId;
pub fn stars() -> Vec<(StarId, StarFunction)> { vec_all_days!(2015) }
