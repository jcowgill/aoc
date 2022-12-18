use crate::direction::Direction;
use nalgebra::Vector2;
use num::Zero;
use std::collections::HashSet;

fn parse_input(input: &str) -> impl Iterator<Item = (Direction, i32)> + '_ {
    input.lines().map(|l| {
        let dir = match l.chars().next().unwrap() {
            'U' => Direction::North,
            'D' => Direction::South,
            'L' => Direction::West,
            'R' => Direction::East,
            _ => panic!("invalid direction"),
        };

        (dir, l.split_at(2).1.parse::<i32>().unwrap())
    })
}

fn simulate<const N: usize>(input: &str) -> String {
    let mut knots = [Vector2::<i32>::zero(); N];
    let mut seen = HashSet::new();
    seen.insert(Vector2::zero());

    for (dir, len) in parse_input(input) {
        for _ in 0..len {
            knots[0] += dir.to_vec();

            for i in 1..N {
                let off = knots[i - 1] - knots[i];
                if off.x.abs() >= 2 || off.y.abs() >= 2 {
                    knots[i] += off.map(|c| c.signum());

                    if i == N - 1 {
                        seen.insert(knots[N - 1]);
                    }
                } else {
                    break;
                }
            }
        }
    }

    seen.len().to_string()
}

pub fn star1(input: &str) -> String {
    simulate::<2>(input)
}

pub fn star2(input: &str) -> String {
    simulate::<10>(input)
}
