//! AOC entry point
//!
//! This module handles command line arguments, collects together all the available stars, and runs
//! the chosen one using data supplied via standard input.

use aoclib::{StarId, all_stars};
use std::env;
use std::io::{self, Read};

/// Prints the usage message for this program
fn print_usage() {
    eprintln!("aoc <star name>");
    eprintln!(" runs the given star (pass input via stdin)");
    eprintln!("aoc --list");
    eprintln!(" prints the list of available stars");
}

fn main() {
    let stars = all_stars();
    let args: Vec<String> = env::args().collect();
    std::process::exit(
        match args.len() {
            0 | 1 => { print_usage(); 0 },
            2 => {
                if let Ok(id) = args[1].parse::<StarId>() {
                    if let Ok(index) = stars.binary_search_by(|probe| probe.0.cmp(&id)) {
                        // Consume all of stdin and run the star!
                        let mut stdin = String::new();
                        match io::stdin().read_to_string(&mut stdin) {
                            Ok(_) => {
                                println!("{}", stars[index].1(stdin.trim_right()));
                                0
                            },
                            Err(e) => {
                                eprintln!("{}: read error: {}", args[0], e);
                                1
                            }
                        }
                    } else {
                        eprintln!("{}: unimplemented star \"{}\"", args[0], args[1]);
                        1
                    }
                } else {
                    // Try known arguments
                    match args[1].as_ref() {
                        "--help" | "-h" => { print_usage(); 0 },
                        "--list" | "-l" => {
                            for (id, _) in stars {
                                println!("{}", id);
                            }
                            0
                        },
                        _ => {
                            eprintln!("{}: unknown argument \"{}\"", args[0], args[1]);
                            1
                        }
                    }
                }
            },
            _ => { eprintln!("{}: too many arguments", args[0]); 1 }
        }
    );
}
