//! Main AOC library functions
//!  This library imports all AOC star implementations and provides various global functions

use std::iter::Cycle;
use std::iter::Zip;

mod direction;
mod duplicate;
#[macro_use] mod macros;
mod vector;

mod yr2015;
mod yr2017;

/// Function type for all star functions
pub type StarFunction = fn (&str) -> String;

/// Type returned by year stars() functions.
///  The vector is always sorted by name and maps star names to implementations.
type StarVector = Vec<(&'static str, StarFunction)>;

/// Returns a vector containing all star maps
fn star_maps() -> Vec<(&'static str, StarVector)> {
    vec![
        ("2015", yr2015::stars()),
        ("2017", yr2017::stars()),
    ]
}

/// Returns the star function with the given name
pub fn star_function(name: &str) -> Option<StarFunction> {
    let maps = star_maps();

    // Split name into year and star parts, then try to find it in the star maps
    let parts: Vec<&str> = name.splitn(2, '-').collect();
    if parts.len() == 2 {
        match maps.binary_search_by(|probe| probe.0.cmp(parts[0])) {
            Ok(i_yr) => {
                match maps[i_yr].1.binary_search_by(|probe| probe.0.cmp(parts[1])) {
                    Ok(i_func) => Some(maps[i_yr].1[i_func].1),
                    _ => None
                }
            },
            _ => None
        }
    } else {
        None
    }
}

/// Returns a list of all available stars
pub fn list_stars() -> Vec<String> {
    let mut names = Vec::new();
    for (year, year_stars) in star_maps() {
        for (name, _) in year_stars {
            names.push(year.to_owned() + "-" + name);
        }
    }

    names
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
