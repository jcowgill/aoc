//! Main AOC library functions
//!  This library imports all AOC star implementations and provides various global functions

#![forbid(unsafe_code)]

use std::fmt;
use std::iter::Cycle;
use std::iter::Zip;
use std::str::FromStr;

mod direction;
mod duplicate;
#[macro_use] mod macros;
mod vector;

mod yr2015;
mod yr2017;
mod yr2018;

/// Function type for all star functions
pub type StarFunction = fn (&str) -> String;

/// Uniquely identifies a star
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StarId
{
    pub year: u16,
    pub day: u8,
    pub star: u8,
}

impl fmt::Display for StarId
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}-{:02}-{}", self.year, self.day, self.star)
    }
}

impl FromStr for StarId
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part_iter = s.splitn(3, '-');
        if let Some(Some(year)) = part_iter.next().map(|s| s.parse().ok()) {
            if let Some(Some(day)) = part_iter.next().map(|s| s.parse().ok()) {
                if let Some(Some(star)) = part_iter.next().map(|s| s.parse().ok()) {
                    return Ok(StarId { year: year, day: day, star: star });
                }
            }
        }

        Err(())
    }
}

/// Returns a vector containing all stars sorted by id
pub fn all_stars() -> Vec<(StarId, StarFunction)> {
    let mut result = Vec::new();
    result.append(&mut yr2015::stars());
    result.append(&mut yr2017::stars());
    result.append(&mut yr2018::stars());
    result
}

/// Returns the star function with the given id
pub fn star_function(id: StarId) -> Option<StarFunction> {
    let stars = all_stars();
    if let Ok(index) = stars.binary_search_by(|probe| probe.0.cmp(&id)) {
        return Some(stars[index].1);
    }

    None
}

#[cfg(test)]
mod tests
{
    use super::*;

    /// Tests that all_stars returns sorted and unique stars
    #[test]
    fn all_stars_sorted() {
        for window in all_stars().windows(2) {
            assert!(window[0].0 < window[1].0);
        }
    }
}


// ========================================================================

/// Returns the cartesian product of two iterators
pub fn cartesian_product<A, B>(a: A, b: B) -> Zip<duplicate::Duplicate<A>, Cycle<B>>
    where A: Iterator, A::Item: Clone, B: Clone + Iterator {
    // Return an iterator made up of duplicating the items in a b times and
    // zipping that up with b cycled a times
    duplicate::duplicate(a, b.clone().count()).zip(b.cycle())
}

/// Identity function - returns given input
pub fn id<T>(v: T) -> T { v }

/// Applies a function combining two heterogeneous tuples
pub fn apply_tuple2<F: Fn(A, B) -> R, A, B, R>(func: F, left: (A, A), right: (B, B)) -> (R, R) {
    (func(left.0, right.0), func(left.1, right.1))
}
