use std::collections::HashSet;

use itertools::Itertools;
use nalgebra::Vector2;
use num::Zero;

const NEIGHBORS: [Vector2<i32>; 6] = [
    Vector2::new(-1, 0),
    Vector2::new(1, 0),
    Vector2::new(0, 1),
    Vector2::new(1, 1),
    Vector2::new(-1, -1),
    Vector2::new(0, -1),
];

fn parse_tile(line: &str) -> Vector2<i32> {
    line.chars()
        .fold((Vector2::zero(), false), |(acc, comp), c| match (comp, c) {
            (_, 'e') => (acc + Vector2::new(1, 0), false),
            (true, 'w') => (acc, false),
            (false, 'w') => (acc - Vector2::new(1, 0), false),
            (false, 's') => (acc + Vector2::new(0, 1), true),
            (false, 'n') => (acc - Vector2::new(1, 1), true),
            _ => panic!("parse error"),
        })
        .0
}

fn parse_input(input: &str) -> impl Iterator<Item = Vector2<i32>> {
    input
        .lines()
        .map(parse_tile)
        .counts()
        .into_iter()
        .filter(|&(_, c)| c % 2 == 1)
        .map(|(pos, _)| pos)
}

pub fn star1(input: &str) -> String {
    parse_input(input).count().to_string()
}

pub fn star2(input: &str) -> String {
    let mut state: HashSet<_> = parse_input(input).collect();
    for _ in 0..100 {
        let mut white_check = HashSet::new();
        let mut new_state: HashSet<_> = state
            .iter()
            .copied()
            .filter(|&tile| {
                let mut neighbor_count = 0;
                for off in NEIGHBORS.iter() {
                    if state.contains(&(tile + off)) {
                        neighbor_count += 1;
                    } else {
                        white_check.insert(tile + off);
                    }
                }

                neighbor_count == 1 || neighbor_count == 2
            })
            .collect();

        new_state.extend(white_check.into_iter().filter(|tile| {
            NEIGHBORS
                .iter()
                .filter(|&n| state.contains(&(tile + n)))
                .count()
                == 2
        }));
        state = new_state;
    }
    state.len().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_tile() {
        assert_eq!(parse_tile("w"), Vector2::new(-1, 0));
        assert_eq!(parse_tile("e"), Vector2::new(1, 0));
        assert_eq!(parse_tile("sw"), Vector2::new(0, 1));
        assert_eq!(parse_tile("se"), Vector2::new(1, 1));
        assert_eq!(parse_tile("nw"), Vector2::new(-1, -1));
        assert_eq!(parse_tile("ne"), Vector2::new(0, -1));
        assert_eq!(parse_tile("seswneswswsenwwnwse"), Vector2::new(0, 3));
    }
}
