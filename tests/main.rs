/// Main testing entry point
use aoclib::{StarId, star_function};
use rstest::rstest;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Reads an entire file into a string
fn read_whole_file(path: &Path) -> io::Result<String> {
    let mut result = String::new();
    File::open(path)?.read_to_string(&mut result)?;
    Ok(result)
}

#[rstest]
fn test_star(#[files("tests/data/**/*.out.*")] out_file: PathBuf) {
    // Parse filename
    let parts: Vec<_> = out_file
        .iter()
        .rev()
        .take(3)
        .map(|s| s.to_str().expect("non utf-8 path name"))
        .collect();
    let name_parts: Vec<_> = parts[0].split('.').collect();
    assert_eq!(name_parts.len(), 3);

    let id = StarId {
        year: parts[2].parse().unwrap(),
        day: parts[1].parse().unwrap(),
        star: name_parts[2].parse().unwrap(),
    };

    let in_file = out_file.with_file_name(format!("{}.in", name_parts[0]));
    let in_file_specific = in_file.with_added_extension(name_parts[2]);

    // Read files
    let input_data = read_whole_file(if in_file_specific.exists() {
        &in_file_specific
    } else {
        &in_file
    })
    .expect("failed to read input file");
    let output_data = read_whole_file(&out_file).expect("failed to read output file");

    // Execute test
    let func = star_function(id).expect("star not found");
    println!(" running \"{}\" on {:?}...", id, out_file);
    assert_eq!(output_data.trim(), func(input_data.trim_end()));
}
