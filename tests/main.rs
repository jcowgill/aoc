/// Main testing entry point

extern crate aoclib;

use aoclib::{StarFunction, list_stars, star_function};
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

/// Checks the star list contains the same set of stars as the test directory does
#[test]
fn test_star_list() {
    let mut stars = list_stars_in_directory(get_data_dir().as_ref());
    let names: Vec<String> = stars.drain(..).map(|(name, _)| name).collect();
    assert_eq!(names, list_stars());
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
fn test_star(func: StarFunction, data_path: &Path) {
    // Given path must be a directory
    assert!(data_path.is_dir());

    // Generate a list of test cases by scanning the data directory
    let mut tests = Vec::new();
    for entry_result in data_path.read_dir().unwrap() {
        let entry = entry_result.unwrap();
        let mut path = entry.path();

        assert!(entry.file_type().unwrap().is_file());
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
        assert_eq!(func(input_data.as_ref()).trim(), output_data.trim());
    }
}

/// Checks the star implementations according to the test directory data
///  Ignores missing stars (unlike above test)
#[test]
fn test_star_implementations() {
    let stars = list_stars_in_directory(get_data_dir().as_ref());
    for (name, path) in stars {
        match star_function(name.as_ref()) {
            Some(func) => test_star(func, path.as_path()),
            None => ()
        }
    }
}
