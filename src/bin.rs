//! AOC entry point
//!
//! This module handles command line arguments, collects together all the available stars, and runs
//! the chosen one using data supplied via standard input.

extern crate aoclib;

use aoclib::{list_stars, star_function};
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
                                println!("{}", func(stdin.trim_right()));
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
