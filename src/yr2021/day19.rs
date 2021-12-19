use itertools::Itertools;
use nalgebra::Vector3;
use std::collections::HashSet;

use crate::vector::VectorExt;

type Point = Vector3<i32>;

/// Enumerates all transforms such that
///  f(b) = a, and f applies one of the 24 rotational symmetries of a cube
fn enumerate_transforms(
    a: Point,
    b: Point,
) -> impl Iterator<Item = impl Fn(Point) -> Point + Clone> {
    (0..24).map(move |i| {
        move |point: Point| {
            let r = move |p: Point| match i {
                0 => p,
                1 => Point::new(p.x, -p.y, -p.z),
                2 => Point::new(p.x, -p.z, p.y),
                3 => Point::new(p.x, p.z, -p.y),
                4 => Point::new(-p.x, -p.y, p.z),
                5 => Point::new(-p.x, p.y, -p.z),
                6 => Point::new(-p.x, p.z, p.y),
                7 => Point::new(-p.x, -p.z, -p.y),

                8 => Point::new(p.y, p.z, p.x),
                9 => Point::new(p.y, -p.z, -p.x),
                10 => Point::new(p.y, -p.x, p.z),
                11 => Point::new(p.y, p.x, -p.z),
                12 => Point::new(-p.y, -p.z, p.x),
                13 => Point::new(-p.y, p.z, -p.x),
                14 => Point::new(-p.y, p.x, p.z),
                15 => Point::new(-p.y, -p.x, -p.z),

                16 => Point::new(p.z, p.x, p.y),
                17 => Point::new(p.z, -p.x, -p.y),
                18 => Point::new(p.z, -p.y, p.x),
                19 => Point::new(p.z, p.y, -p.x),
                20 => Point::new(-p.z, -p.x, p.y),
                21 => Point::new(-p.z, p.x, -p.y),
                22 => Point::new(-p.z, p.y, p.x),
                23 => Point::new(-p.z, -p.y, -p.x),
                _ => unreachable!(),
            };

            r(point) - r(b) + a
        }
    })
}

/// Parses input returning a list of scanners. Each scanner contains a
/// list of beacons detected by it.
fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .filter(|l| l.contains(','))
                .map(|l| Point::from_iterator(l.split(',').map(|c| c.trim().parse().unwrap())))
                .collect()
        })
        .collect()
}

/// Find a stations and it's transform which can be used to add becons
/// to the joined set
fn find_transform(
    stations: &[Vec<Point>],
    joined_becons: &HashSet<Point>,
) -> Option<(usize, impl Fn(Point) -> Point + Clone)> {
    for (i, station) in stations.iter().enumerate() {
        // Test every point in the station againsted the joined
        // points to see if this station overlaps given some
        // transform
        for &jp in joined_becons {
            for &sp in station {
                for trans in enumerate_transforms(jp, sp) {
                    // Are there 12 beacons in the overlap?
                    if station
                        .iter()
                        .filter(|&&p| joined_becons.contains(&trans(p)))
                        .count()
                        >= 5
                    {
                        // Yay - found a correct transform.
                        return Some((i, trans));
                    }
                }
            }
        }
    }

    None
}

/// Resolves the beacons described by the input
///
/// Returns (Beacons, Station centers in arbitrary order)
fn resolve_beacons(input: &str) -> (HashSet<Point>, Vec<Point>) {
    // Stations still left to process
    let mut stations = parse_input(input);
    let mut station_centers = Vec::with_capacity(stations.len());

    // Becons part of the final becon set (initially station 0)
    let mut joined_becons: HashSet<Point> = stations.swap_remove(0).into_iter().collect();

    while !stations.is_empty() {
        if let Some((station, trans)) = find_transform(&stations, &joined_becons) {
            // Move station into the joined set using the found transform.
            joined_becons.extend(stations.swap_remove(station).into_iter().map(trans.clone()));
            station_centers.push(trans(Point::zeros()));
        } else {
            // This can happen if there's two sets of non-overlapping stations
            panic!("disjoint stations");
        }
    }

    (joined_becons, station_centers)
}

pub fn star1(input: &str) -> String {
    resolve_beacons(input).0.len().to_string()
}

pub fn star2(input: &str) -> String {
    resolve_beacons(input)
        .1
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a - b).taxicab_norm())
        .max()
        .unwrap_or(0)
        .to_string()
}
