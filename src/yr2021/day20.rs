use itertools::Itertools;
use nalgebra::Vector2;
use std::collections::HashSet;

type Point = Vector2<i32>;

#[derive(Clone, Debug)]
struct Grid {
    points: HashSet<Point>,
    flipped: bool,
    bounds1: Point,
    bounds2: Point,
    algo: [u8; 64],
}

impl Grid {
    /// Returns true if the bit for the given algorithm value is set
    fn algo_bit(&self, value: u16) -> bool {
        (self.algo[usize::from(value / 8)] & (1 << (value % 8))) != 0
    }

    /// Returns true if the given point is within the points set boundary
    fn is_in_bounds(&self, point: Point) -> bool {
        (self.bounds1.x..self.bounds2.x).contains(&point.x)
            && (self.bounds1.y..self.bounds2.y).contains(&point.y)
    }

    /// Returns the (possibly flipped) value at the given position
    fn get_point(&self, point: Point) -> bool {
        (self.is_in_bounds(point) && self.points.contains(&point)) ^ self.flipped
    }

    /// Performs one step of the algorithm
    fn step(&self) -> Grid {
        let mut points = HashSet::new();
        let flipped = self.algo_bit(0) ^ self.flipped;
        let bounds1 = self.bounds1 - Point::new(1, 1);
        let bounds2 = self.bounds2 + Point::new(1, 1);

        for y in bounds1.y..bounds2.y {
            for x in bounds1.x..bounds2.x {
                let middle = Point::new(x, y);
                let mut value = 0;
                for ysub in -1..=1 {
                    for xsub in -1..=1 {
                        value <<= 1;
                        value |= self.get_point(middle + Point::new(xsub, ysub)) as u16;
                    }
                }

                if self.algo_bit(value) ^ flipped {
                    points.insert(middle);
                }
            }
        }

        Grid {
            points,
            flipped,
            bounds1,
            bounds2,
            algo: self.algo,
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let (algo_str, grid_str) = input.split_once("\n\n").unwrap();
    let mut algo = [0; 64];
    for (i, chunk) in algo_str
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .chunks(8)
        .into_iter()
        .enumerate()
    {
        algo[i] = chunk.fold(0, |acc, c| acc >> 1 | ((c == '#') as u8) << 7);
    }

    let points: HashSet<_> = grid_str
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Point::new(x as i32, y as i32))
        })
        .collect();

    let bounds2 = Point::new(
        points.iter().map(|p| p.x).max().unwrap_or(0) + 1,
        points.iter().map(|p| p.y).max().unwrap_or(0) + 1,
    );

    Grid {
        points,
        flipped: false,
        bounds1: Point::zeros(),
        bounds2,
        algo,
    }
}

fn star_common(input: &str, steps: usize) -> String {
    (0..steps)
        .fold(parse_input(input), |grid, _| grid.step())
        .points
        .len()
        .to_string()
}

pub fn star1(input: &str) -> String {
    star_common(input, 2)
}

pub fn star2(input: &str) -> String {
    star_common(input, 50)
}
