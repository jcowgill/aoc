/// Main testing entry point

extern crate aoclib;

use aoclib::{StarFunction, star_function};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Returns the data directory to use
fn get_data_dir() -> String {
    option_env!("CARGO_MANIFEST_DIR").unwrap_or(".").to_string() + "/tests/data"
}

/// Generates a sorted list of stars available in the given data directory
///  Panics if an IO error occurs
fn list_stars_in_directory(data_dir: &str) -> Vec<(String, PathBuf)> {
    let mut names = Vec::new();
    for year_entry_result in Path::new(data_dir).read_dir().unwrap() {
        let year_entry = year_entry_result.unwrap();
        let year_name = year_entry.file_name().into_string().unwrap();
        for star_entry_result in year_entry.path().read_dir().unwrap() {
            let star_entry = star_entry_result.unwrap();
            let star_name = star_entry.file_name().into_string().unwrap();
            names.push((year_name.clone() + "-" + star_name.as_ref(), star_entry.path()));
        }
    }

    names.sort_unstable();
    names
}

/// Reads the entire contents of a test data file
///  Panics on error
fn read_test_file(test: &PathBuf, extension: &str) -> String {
    let mut path = test.clone();
    path.set_extension(extension);

    let mut result = String::new();
    File::open(path).unwrap().read_to_string(&mut result).unwrap();
    result
}

/// Tests that the given star function operates correctly
///  Panics on error
fn test_star(name: &str, func: StarFunction, data_path: &Path) {
    // Given path must be a directory
    assert!(data_path.is_dir());

    // Generate a list of test cases by scanning the data directory
    let mut tests = Vec::new();
    for entry_result in data_path.read_dir().unwrap() {
        let mut path = entry_result.unwrap().path();
        assert!(path.extension().unwrap() == "in" || path.extension().unwrap() == "out");

        path.set_extension("dummy");
        tests.push(path);
    }
    tests.sort_unstable();
    tests.dedup();

    // Run func on each test
    for test in tests {
        let input_data = read_test_file(&test, "in");
        let output_data = read_test_file(&test, "out");

        println!(" running \"{}\" on {:?}...", name, test.file_stem().unwrap());
        assert_eq!(output_data.trim(), func(input_data.trim_right()));
    }
}

/// Helper function for gen_tests macro
fn gen_tests_helper(year: &str, day: &str) {
    assert!(year.starts_with("yr"));
    assert!(day.starts_with("day"));
    let prefix = format!("{}-{}-", year.split_at(2).1, day.split_at(3).1);

    for (name, path) in list_stars_in_directory(get_data_dir().as_ref()) {
        if name.starts_with(&prefix[..]) {
            match star_function(name.as_ref()) {
                Some(func) => test_star(name.as_ref(), func, path.as_path()),
                None => ()
            }
        }
    }
}

/// Macro which generates a list of tests for specified days in a year
macro_rules! gen_tests_days {
    ( $year:ident, $( $day:ident ),* ) => {
        $(
            #[test]
            fn $day() {
                gen_tests_helper(stringify!($year), stringify!($day))
            }
        )*
    }
}

/// Macro which generates a list of tests for a specific year
macro_rules! gen_tests {
    ( $year:ident ) => {
        mod $year {
            use super::gen_tests_helper;
            gen_tests_days!($year,
                            day01, day02, day03, day04, day05, day06, day07, day08, day09,
                            day10, day11, day12, day13, day14, day15, day16, day17,
                            day18, day19, day20, day21, day22, day23, day24, day25);
        }
    }
}

gen_tests!(yr2015);
gen_tests!(yr2017);
gen_tests!(yr2018);
