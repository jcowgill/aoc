/// Main testing entry point
use aoclib::{star_function, StarId};
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Returns the test data directory containing all the files for the given day
fn get_day_data_dir(year: u16, day: u8) -> PathBuf {
    [
        option_env!("CARGO_MANIFEST_DIR").unwrap_or("."),
        "tests",
        "data",
        &format!("{:04}", year),
        &format!("{:02}", day),
    ]
    .iter()
    .collect()
}

/// Reads an entire file into a string
fn read_whole_file(path: &Path) -> io::Result<String> {
    let mut result = String::new();
    File::open(path)?.read_to_string(&mut result)?;
    Ok(result)
}

/// Run all tests for the given day
fn test_day(year: u16, day: u8) {
    // Skip this day if the data directory is missing
    if let Ok(dir_iter) = get_day_data_dir(year, day).read_dir() {
        let mut files: Vec<PathBuf> = dir_iter.map(|entry| entry.unwrap().path()).collect();
        files.sort_by(|a, b| a.file_name().unwrap().cmp(b.file_name().unwrap()));

        // Iterate over each output file
        for file in files.iter() {
            let file_name_parts: Vec<&str> = file
                .file_name()
                .unwrap()
                .to_str()
                .expect("non utf-8 filename")
                .splitn(3, '.')
                .collect();

            if file_name_parts[1] == "out" {
                // Extract star to be processed
                let id = StarId {
                    year,
                    day,
                    star: file_name_parts[2].parse().expect("invalid output filename"),
                };

                // Find the corresponding input file
                let input_nonspecific = file_name_parts[0].to_owned() + ".in";
                let input_specific = input_nonspecific.to_owned() + "." + file_name_parts[2];

                let input_path = files
                    .iter()
                    .find(|p| p.file_name().unwrap().to_str().unwrap_or("") == input_specific)
                    .or_else(|| {
                        files.iter().find(|p| {
                            p.file_name().unwrap().to_str().unwrap_or("") == input_nonspecific
                        })
                    })
                    .expect("failed to find input test");

                // Read input files
                let input_data = read_whole_file(input_path).expect("failed to read input file");
                let output_data = read_whole_file(file).expect("failed to read output file");

                // Execute test
                let func = star_function(id).expect("star not found");
                println!(" running \"{}\" on {:?}...", id, file_name_parts[0]);
                assert_eq!(output_data.trim(), func(input_data.trim_end()));
            }
        }
    }
}

/// Helper function for gen_tests macro
fn gen_tests_helper(year: &str, day: &str) {
    assert!(year.starts_with("yr"));
    assert!(day.starts_with("day"));

    test_day(
        year.split_at(2).1.parse().unwrap(),
        day.split_at(3).1.parse().unwrap(),
    );
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
            gen_tests_days!(
                $year, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11,
                day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23,
                day24, day25
            );
        }
    };
}

gen_tests!(yr2015);
gen_tests!(yr2017);
gen_tests!(yr2018);
gen_tests!(yr2020);
gen_tests!(yr2021);
gen_tests!(yr2022);
gen_tests!(yr2025);
