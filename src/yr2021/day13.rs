use std::{cmp::Ordering, iter::once};

use itertools::Itertools;
use nalgebra::Vector2;

fn parse_input(input: &str) -> (Vec<Vector2<i32>>, Vec<(u8, i32)>) {
    let (dots_str, folds_str) = input.split_once("\n\n").unwrap();
    (
        dots_str
            .lines()
            .map(|l| Vector2::from_iterator(l.split(',').map(|v| v.parse().unwrap())))
            .collect(),
        folds_str
            .lines()
            .map(|l| {
                let (axis_str, line_str) = l
                    .strip_prefix("fold along ")
                    .unwrap()
                    .split_once('=')
                    .unwrap();
                (
                    u8::from(axis_str != "x"),
                    line_str.parse().unwrap(),
                )
            })
            .collect(),
    )
}

fn do_folds(input: &str, max_folds: usize) -> impl Iterator<Item = Vector2<i32>> {
    let (mut dots, folds) = parse_input(input);
    for (axis, line) in folds.into_iter().take(max_folds) {
        for i in 0..dots.len() {
            match dots[i][axis as usize].cmp(&line) {
                Ordering::Less => (),
                Ordering::Equal => {
                    dots.swap_remove(i);
                }
                Ordering::Greater => dots[i][axis as usize] = 2 * line - dots[i][axis as usize],
            }
        }
    }

    dots.sort_unstable_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
    dots.dedup();
    dots.into_iter()
}

pub fn star1(input: &str) -> String {
    do_folds(input, 1).count().to_string()
}

pub fn star2(input: &str) -> String {
    let mut result = String::new();
    for (prev, dot) in once(Vector2::zeros())
        .chain(do_folds(input, usize::MAX))
        .tuple_windows()
    {
        let char_diff = dot - prev;
        if char_diff.y > 0 {
            (0..char_diff.y).for_each(|_| result.push('\n'));
            (0..dot.x).for_each(|_| result.push(' '));
        } else {
            (1..char_diff.x).for_each(|_| result.push(' '));
        }
        result.push('#');
    }
    result
}
