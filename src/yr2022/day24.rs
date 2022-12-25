use crate::direction::Direction;
use nalgebra::{DMatrix, Vector2};
use std::collections::HashSet;

type Grid = DMatrix<Option<Direction>>;

fn parse_input(input: &str) -> Grid {
    let width = input.lines().next().unwrap().len() - 2;
    let height = input.lines().count() - 2;
    DMatrix::from_row_iterator(
        height,
        width,
        input
            .lines()
            .filter(|line| !line.contains("##"))
            .flat_map(|line| {
                line.chars().filter_map(|c| match c {
                    '#' => None,
                    '.' => Some(None),
                    '^' => Some(Some(Direction::North)),
                    'v' => Some(Some(Direction::South)),
                    '>' => Some(Some(Direction::East)),
                    '<' => Some(Some(Direction::West)),
                    _ => panic!("invalid character {}", c),
                })
            }),
    )
}

fn get_wrapped(grid: &Grid, pos: Vector2<i32>) -> Option<Direction> {
    grid[(
        pos.y.rem_euclid(grid.nrows() as i32) as usize,
        pos.x.rem_euclid(grid.ncols() as i32) as usize,
    )]
}

fn is_open(grid: &Grid, pos: Vector2<i32>, depth: u32) -> bool {
    pos.x >= 0
        && pos.y >= 0
        && (pos.x as usize) < grid.ncols()
        && (pos.y as usize) < grid.nrows()
        && Direction::iter()
            .all(|d| get_wrapped(grid, pos + d.reverse().to_vec_neg() * (depth as i32)) != Some(d))
}

fn solve(grid: &Grid, start: Vector2<i32>, end: Vector2<i32>, initial_depth: u32) -> u32 {
    let mut open = HashSet::new();
    open.insert(start);

    for depth in initial_depth.. {
        let mut next_open = HashSet::new();
        for pos in open {
            // Try each direction
            for dir in Direction::iter() {
                let new_pos = pos + dir.to_vec_neg();
                if new_pos == end {
                    return depth + 1;
                }

                if is_open(grid, new_pos, depth + 1) {
                    next_open.insert(new_pos);
                }
            }

            // Wait if possible
            if is_open(grid, pos, depth + 1) {
                next_open.insert(pos);
            }
        }

        next_open.insert(start);
        open = next_open;
    }

    unreachable!()
}

fn solve_trips(input: &str, trips: u32) -> String {
    let grid = parse_input(input);
    let start = Vector2::new(0, -1);
    let end = Vector2::new(grid.ncols() as i32 - 1, grid.nrows() as i32);

    (0..trips)
        .fold(0, |depth, i| {
            if i % 2 == 0 {
                solve(&grid, start, end, depth)
            } else {
                solve(&grid, end, start, depth)
            }
        })
        .to_string()
}

pub fn star1(input: &str) -> String {
    solve_trips(input, 1)
}

pub fn star2(input: &str) -> String {
    solve_trips(input, 3)
}
