//! Year 2021 modules

#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::maybe_infinite_iter,
    clippy::must_use_candidate,
    clippy::similar_names,
    clippy::unnested_or_patterns
)]

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use crate::StarFunction;
use crate::StarId;
pub fn stars() -> Vec<(StarId, StarFunction)> {
    vec_all_days!(2021)
}
