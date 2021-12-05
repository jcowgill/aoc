use itertools::Itertools;
use num::integer::gcd;

use crate::vector::Vector2;

/// A line in parametric vector form (line = pos + t * dir, where 0 <= t <= 1)
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Line {
    pos: Vector2<i32>,
    dir: Vector2<i32>,
}

impl Line {
    /// Returns an iterator over all points on this line
    fn points(self) -> impl Iterator<Item = Vector2<i32>> {
        let gcd = gcd(self.dir.x, self.dir.y);
        let step = self.dir / gcd;

        (0..gcd + 1).map(move |i| self.pos + step * i)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Line> + '_ {
    input.lines().map(|l| {
        let (start, end) = l.split_once("->").unwrap();
        let (x1, y1) = start.trim().split_once(',').unwrap();
        let (x2, y2) = end.trim().split_once(',').unwrap();
        let pos = Vector2 {
            x: x1.parse().unwrap(),
            y: y1.parse().unwrap(),
        };
        let dir = Vector2 {
            x: x2.parse().unwrap(),
            y: y2.parse().unwrap(),
        } - pos;
        Line { pos, dir }
    })
}

pub fn star1(input: &str) -> String {
    parse_input(input)
        .filter(|l| l.dir.x == 0 || l.dir.y == 0)
        .flat_map(Line::points)
        .duplicates()
        .count()
        .to_string()
}

pub fn star2(input: &str) -> String {
    parse_input(input)
        .flat_map(Line::points)
        .duplicates()
        .count()
        .to_string()
}
