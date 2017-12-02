//! AOC entry point
//!
//! This module handles command line arguments, collects together all the available stars, and runs
//! the chosen one using data supplied via standard input.

mod aoclib;
mod yr2017;

use aoclib::{StarFunction, StarVector};
use std::env;
use std::io::{self, Read};

/// Returns a vector containing all star maps
fn star_maps() -> Vec<(&'static str, StarVector)> {
    vec![
        ("2017", yr2017::stars())
    ]
}

/// Returns the star function with the given name
fn star_function(name: &str) -> Option<StarFunction> {
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
fn list_stars() -> Vec<String> {
    let mut names = Vec::new();
    for (year, year_stars) in star_maps() {
        for (name, _) in year_stars {
            names.push(year.to_owned() + "-" + name);
        }
    }

    names
}

/// Attempts to check all the available stars using data from the given directory (relative to the
/// current directory)
///
/// This function prints its results to standard output and returns a boolean indicating if all the
/// tests passed.
fn check(data_dir: &str) -> bool {
    false
}

/// Prints the usage message for this program
fn print_usage() {
    eprintln!("aoc <star name>");
    eprintln!(" runs the given star (pass input via stdin)");
    eprintln!("aoc --list");
    eprintln!(" prints the list of available stars");
    eprintln!("aoc --check <data dir>");
    eprintln!(" checks all the stars using data in the given directory");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option_arg = if args.len() >= 2 { Some(args[1].as_ref()) } else { None };

    std::process::exit(
        match option_arg {
            Some("--help") | Some("-h") | None => { print_usage(); 0 },
            Some("--list") | Some("-l") => {
                for name in list_stars() {
                    println!("{}", name);
                }
                0
            },
            Some("--check") | Some("-c") | Some ("--test") | Some("-t") => {
                if args.len() == 3 {
                    if check(args[2].as_ref()) { 0 } else { 1 }
                } else {
                    eprintln!("run --check with one data directory argument");
                    1
                }
            },
            Some(arg) => {
                if args.len() != 2 {
                    eprintln!("too many arguments");
                    1
                } else if arg.starts_with("--") {
                    eprintln!("unknown option \"{}\"", arg);
                    1
                } else {
                    match star_function(&args[1]) {
                        Some(func) => {
                            // Consume all of stdin and run the star!
                            let mut stdin = String::new();
                            match io::stdin().read_to_string(&mut stdin) {
                                Ok(_) => {
                                    println!("{}", func(&stdin));
                                    0
                                },
                                Err(e) => {
                                    eprintln!("read error: {}", e);
                                    1
                                }
                            }
                        }
                        None => {
                            eprintln!("unknown star \"{}\"", arg);
                            1
                        }
                    }
                }
            },
        }
    );
}
