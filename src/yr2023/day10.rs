use std::collections::{HashSet, VecDeque};

use nalgebra::{try_convert, DMatrix, Vector2};

use crate::direction::Direction;

const PIPES: [(u8, [Direction; 2]); 6] = [
    (b'|', [Direction::North, Direction::South]),
    (b'-', [Direction::East, Direction::West]),
    (b'L', [Direction::North, Direction::East]),
    (b'J', [Direction::North, Direction::West]),
    (b'7', [Direction::South, Direction::West]),
    (b'F', [Direction::South, Direction::East]),
];

fn grid_get_pipe(grid: &DMatrix<u8>, pos: Vector2<i32>) -> Option<[Direction; 2]> {
    try_convert(pos)
        .and_then(|posus: Vector2<usize>| grid.get((posus.y, posus.x)).copied())
        .and_then(|b| PIPES.iter().find(|&&(p, _)| b == p))
        .map(|&(_, dirs)| dirs)
}

fn parse_input(input: &str) -> DMatrix<u8> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    DMatrix::from_row_iterator(height, width, input.bytes().filter(u8::is_ascii_graphic))
}

fn iterate_loop(grid: &DMatrix<u8>) -> impl Iterator<Item = (Vector2<i32>, Direction)> + '_ {
    let spos = grid.iter().position(|&b| b == b'S').unwrap();
    let vec_spos = Vector2::new((spos / grid.nrows()) as i32, (spos % grid.nrows()) as i32);

    let first_dir = Direction::iter()
        .find(|&dir| {
            grid_get_pipe(grid, vec_spos + dir.to_vec_neg())
                .filter(|dirs| dirs.contains(&dir.reverse()))
                .is_some()
        })
        .unwrap();

    let mut state = Some((vec_spos + first_dir.to_vec_neg(), first_dir));
    std::iter::from_fn(move || {
        if let prev @ Some((pos, last_dir)) = state {
            state = grid_get_pipe(grid, pos)
                .and_then(|dirs| dirs.into_iter().find(|&d| d != last_dir.reverse()))
                .map(|d| (pos + d.to_vec_neg(), d));
            prev
        } else {
            None
        }
    })
}

pub fn star1(input: &str) -> String {
    (iterate_loop(&parse_input(input)).count() / 2).to_string()
}

pub fn star2(input: &str) -> String {
    let grid = parse_input(input);
    let mut points = HashSet::new();
    let mut left = HashSet::new();
    let mut right = HashSet::new();

    for (p, d) in iterate_loop(&grid) {
        let d_rev = d.reverse();
        points.insert(p);
        if let Some(other_d) =
            grid_get_pipe(&grid, p).and_then(|dirs| dirs.into_iter().find(|&other| other != d_rev))
        {
            let mut is_right = false;
            let mut current = d.anticlockwise();
            while current != d_rev {
                if current == other_d {
                    is_right = true;
                } else if is_right {
                    right.insert(p + current.to_vec_neg());
                } else {
                    left.insert(p + current.to_vec_neg());
                }

                current = current.clockwise();
            }
        }
    }

    // The enclosed loop is the side with less points in it
    let enclosed_seeds = if left.len() < right.len() {
        left
    } else {
        right
    };

    // Flood fill all the enclosed points using "points" as the boundary
    let mut open: VecDeque<_> = enclosed_seeds.into_iter().collect();
    let mut closed = HashSet::new();

    while let Some(pos) = open.pop_front() {
        if !closed.contains(&pos) && !points.contains(&pos) {
            assert!(pos.x >= 0 && (pos.x as usize) < grid.ncols());
            assert!(pos.y >= 0 && (pos.y as usize) < grid.nrows());
            closed.insert(pos);
            open.extend(Direction::iter().map(|d| pos + d.to_vec_neg()));
        }
    }

    closed.len().to_string()
}
