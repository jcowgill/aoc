use crate::direction::Direction;
use itertools::Itertools;
use nalgebra::Vector2;
use std::collections::{HashMap, hash_map::Entry};

fn simulate(input: &str, max_rounds: u32) -> (HashMap<Vector2<i32>, Option<Vector2<i32>>>, u32) {
    let mut grid: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (Vector2::new(x as i32, y as i32), None))
        })
        .collect();

    for round in 0..max_rounds {
        let mut directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        directions.rotate_left((round % 4) as usize);

        let mut new_grid: HashMap<_, Option<_>> = HashMap::new();
        for &pos in grid.keys() {
            let valid_dirs = directions.map(|dir| {
                (-1..=1).all(|off| {
                    !grid.contains_key(
                        &(pos + dir.to_vec_neg() + dir.clockwise().to_vec_neg() * off),
                    )
                })
            });

            let new_pos = if valid_dirs == [true; 4] {
                pos
            } else if let Some(i) = valid_dirs.into_iter().position(|f| f) {
                pos + directions[i].to_vec_neg()
            } else {
                pos
            };

            match new_grid.entry(new_pos) {
                Entry::Occupied(mut e) => {
                    // Collision, undo change and force current position
                    if let Some(other) = e.get_mut().take() {
                        assert_ne!(new_pos, other);
                        new_grid.insert(other, Some(other));
                    }

                    new_grid.insert(pos, Some(pos));
                }
                Entry::Vacant(e) => {
                    e.insert(Some(pos));
                }
            }
        }

        new_grid.retain(|_, old| old.is_some());

        if grid == new_grid {
            return (grid, round);
        }

        grid = new_grid;
    }

    (grid, max_rounds)
}

pub fn star1(input: &str) -> String {
    let (grid, _) = simulate(input, 10);
    let (left, right) = grid.keys().map(|k| k.x).minmax().into_option().unwrap();
    let (top, bottom) = grid.keys().map(|k| k.y).minmax().into_option().unwrap();
    ((right - left + 1) * (bottom - top + 1) - grid.len() as i32).to_string()
}

pub fn star2(input: &str) -> String {
    simulate(input, u32::MAX).1.to_string()
}
