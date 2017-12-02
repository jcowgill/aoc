//! Main AOC library functions
//!  This library imports all AOC star implementations and provides various global functions

mod yr2017;

/// Function type for all star functions
pub type StarFunction = fn (&str) -> String;

/// Type returned by year stars() functions.
///  The vector is always sorted by name and maps star names to implementations.
pub type StarVector = Vec<(&'static str, StarFunction)>;

/// Returns a vector containing all star maps
fn star_maps() -> Vec<(&'static str, StarVector)> {
    vec![
        ("2017", yr2017::stars())
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
