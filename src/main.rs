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

/// Prints the usage message for this program
fn print_usage() {
    eprintln!("aoc <star name>");
    eprintln!(" runs the given star (pass input via stdin)");
    eprintln!("aoc --list");
    eprintln!(" prints the list of available stars");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    std::process::exit(
        match args.len() {
            0 | 1 => { print_usage(); 0 },
            2 => {
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
                                eprintln!("{}: read error: {}", args[0], e);
                                1
                            }
                        }
                    }
                    None => {
                        // Try known arguments
                        match args[1].as_ref() {
                            "--help" | "-h" => { print_usage(); 0 },
                            "--list" | "-l" => {
                                for name in list_stars() {
                                    println!("{}", name);
                                }
                                0
                            },
                            _ => {
                                let thing =
                                    if args[1].starts_with("--") { "option" } else { "star" };
                                eprintln!("{}: unknown {} \"{}\"", args[0], thing, args[1]);
                                1
                            }
                        }
                    }
                }
            },
            _ => { eprintln!("{}: too many arguments", args[0]); 1 }
        }
    );
}
