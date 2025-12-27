use std::collections::HashSet;

use itertools::Itertools;
use nalgebra::Vector2;

use crate::direction::Direction;

type Position = Vector2<i32>;
type PositionValue = (Position, u8);

struct Grid {
    data: Vec<u8>,
    width: usize,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let mut lines = input.lines().peekable();
        let width = lines.peek().unwrap().trim().len();
        Grid {
            data: lines
                .flat_map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap() as u8))
                .collect(),
            width,
        }
    }

    fn get(&self, pos: Position) -> Option<u8> {
        if let (Ok(x), Ok(y)) = (usize::try_from(pos.x), usize::try_from(pos.y))
            && x < self.width
        {
            return self.data.get(x + y * self.width).copied();
        }

        None
    }

    fn enumerate(&self) -> impl Iterator<Item = PositionValue> + '_ {
        (0..self.data.len() / self.width).flat_map(|y| {
            (0..self.width)
                .map(move |x| Position::new(x as i32, y as i32))
                .map(|p| (p, self.get(p).unwrap()))
        })
    }

    fn surrounding(&self, pos: Position) -> impl Iterator<Item = PositionValue> + '_ {
        Direction::iter().filter_map(move |d| {
            let new_pos = pos + d.to_vec();
            self.get(new_pos).map(|v| (new_pos, v))
        })
    }

    /// Returns an iterator over all basin low points
    fn low_points(&self) -> impl Iterator<Item = PositionValue> + '_ {
        self.enumerate()
            .filter(|&(pos, value)| self.surrounding(pos).all(|(_, v)| value < v))
    }

    /// Returns the size of the basin centered at the given low point
    fn basin_size(&self, low_point: PositionValue) -> usize {
        let mut open = vec![low_point];
        let mut closed = HashSet::new();

        while let Some((pos, value)) = open.pop() {
            if closed.insert(pos) {
                open.extend(
                    self.surrounding(pos)
                        .filter(|&point| point.1 < 9 && point.1 >= value),
                );
            }
        }

        closed.len()
    }
}

pub fn star1(input: &str) -> String {
    Grid::parse(input)
        .low_points()
        .map(|(_, value)| u32::from(value + 1))
        .sum::<u32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let heightmap = Grid::parse(input);
    heightmap
        .low_points()
        .map(|p| usize::MAX - heightmap.basin_size(p))
        .k_smallest(3)
        .map(|s| usize::MAX - s)
        .product::<usize>()
        .to_string()
}
