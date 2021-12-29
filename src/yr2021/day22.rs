use itertools::Either;
use nalgebra::Vector3;
use regex::Regex;
use std::iter;

/// An arbitrary cuboid such that a.? < b.? for all 3 dimensions
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Cuboid {
    a: Vector3<i32>,
    b: Vector3<i32>,
}

impl Cuboid {
    /// Constructs a new cuboid from two vectors. Returns None if the
    /// cuboid would be degenerate or have zero volume.
    fn new(a: Vector3<i32>, b: Vector3<i32>) -> Option<Cuboid> {
        if a.iter().zip(b.iter()).all(|(l, r)| l < r) {
            Some(Cuboid { a, b })
        } else {
            None
        }
    }

    /// Returns the intersection of two cuboids or None if they don't
    /// intersect
    fn intersection(&self, other: Cuboid) -> Option<Cuboid> {
        Cuboid::new(
            self.a.zip_map(&other.a, i32::max),
            self.b.zip_map(&other.b, i32::min),
        )
    }

    /// Subtracts the intersection of this and another cuboid from
    /// self. Returns a list of the new cuboids making up this one.
    fn subtract_intersection(self, other: Cuboid) -> impl Iterator<Item = Cuboid> {
        if let Some(inter) = self.intersection(other) {
            // Chop remainder into 6 cuboids (2 for each dimension
            // above and below the intersection region)
            Either::Left(
                [
                    // Z
                    Cuboid::new(self.a, Vector3::new(self.b.x, self.b.y, inter.a.z)),
                    Cuboid::new(Vector3::new(self.a.x, self.a.y, inter.b.z), self.b),
                    // Z + Y
                    Cuboid::new(
                        Vector3::new(self.a.x, self.a.y, inter.a.z),
                        Vector3::new(self.b.x, inter.a.y, inter.b.z),
                    ),
                    Cuboid::new(
                        Vector3::new(self.a.x, inter.b.y, inter.a.z),
                        Vector3::new(self.b.x, self.b.y, inter.b.z),
                    ),
                    // Z + Y + X
                    Cuboid::new(
                        Vector3::new(self.a.x, inter.a.y, inter.a.z),
                        Vector3::new(inter.a.x, inter.b.y, inter.b.z),
                    ),
                    Cuboid::new(
                        Vector3::new(inter.b.x, inter.a.y, inter.a.z),
                        Vector3::new(self.b.x, inter.b.y, inter.b.z),
                    ),
                    // Final section (inter.a -> inter.b) is omitted
                    // because we're subtracting it
                ]
                .into_iter()
                .flatten(),
            )
        } else {
            Either::Right(iter::once(self))
        }
    }

    /// Returns the volume of this cuboid
    fn volume(&self) -> u64 {
        (self.b - self.a).fold(1, |acc, v| acc * v as u64)
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = (bool, Cuboid)> + '_ {
    let regex =
        Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
            .unwrap();

    input.lines().map(move |l| {
        let captures = regex.captures(l).unwrap();
        (
            &captures[1] == "on",
            Cuboid {
                a: Vector3::new(
                    captures[2].parse().unwrap(),
                    captures[4].parse().unwrap(),
                    captures[6].parse().unwrap(),
                ),
                b: Vector3::new(
                    captures[3].parse::<i32>().unwrap() + 1,
                    captures[5].parse::<i32>().unwrap() + 1,
                    captures[7].parse::<i32>().unwrap() + 1,
                ),
            },
        )
    })
}

fn star_common(input: &str, filter: impl Fn(Cuboid) -> bool) -> String {
    let mut disjoint_on_cuboids: Vec<Cuboid> = Vec::new();

    for (value, cuboid) in parse_input(input).filter(|&(_, c)| filter(c)) {
        // Subtract intersection from every existing cuboid, then add
        // this cuboid back if it shold be on. This maintaines the
        // "disjoint on" invariant.
        disjoint_on_cuboids = disjoint_on_cuboids
            .into_iter()
            .flat_map(|prev| prev.subtract_intersection(cuboid))
            .collect();

        if value {
            disjoint_on_cuboids.push(cuboid);
        }
    }

    disjoint_on_cuboids
        .into_iter()
        .map(|c| c.volume())
        .sum::<u64>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    star_common(input, |c| c.a.abs().max() <= 50 && c.b.abs().max() <= 51)
}

pub fn star2(input: &str) -> String {
    star_common(input, |_| true)
}
