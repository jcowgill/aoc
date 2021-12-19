use itertools::Itertools;
use nalgebra::Vector3;
use std::collections::{HashMap, HashSet};

use crate::vector::VectorExt;

type Point = Vector3<i32>;

/// Parses input returning a list of scanners. Each scanner contains a
/// list of beacons detected by it.
fn parse_input(input: &str) -> impl Iterator<Item = Vec<Point>> + '_ {
    input.split("\n\n").map(|scanner| {
        scanner
            .lines()
            .filter(|l| l.contains(','))
            .map(|l| Point::from_iterator(l.split(',').map(|c| c.trim().parse().unwrap())))
            .collect()
    })
}

/// List of all octahedral rotation functions
static ROTATIONS: [fn(Point) -> Point; 24] = [
    |p| p,
    |p| Point::new(p.x, -p.y, -p.z),
    |p| Point::new(p.x, -p.z, p.y),
    |p| Point::new(p.x, p.z, -p.y),
    |p| Point::new(-p.x, -p.y, p.z),
    |p| Point::new(-p.x, p.y, -p.z),
    |p| Point::new(-p.x, p.z, p.y),
    |p| Point::new(-p.x, -p.z, -p.y),
    |p| Point::new(p.y, p.z, p.x),
    |p| Point::new(p.y, -p.z, -p.x),
    |p| Point::new(p.y, -p.x, p.z),
    |p| Point::new(p.y, p.x, -p.z),
    |p| Point::new(-p.y, -p.z, p.x),
    |p| Point::new(-p.y, p.z, -p.x),
    |p| Point::new(-p.y, p.x, p.z),
    |p| Point::new(-p.y, -p.x, -p.z),
    |p| Point::new(p.z, p.x, p.y),
    |p| Point::new(p.z, -p.x, -p.y),
    |p| Point::new(p.z, -p.y, p.x),
    |p| Point::new(p.z, p.y, -p.x),
    |p| Point::new(-p.z, -p.x, p.y),
    |p| Point::new(-p.z, p.x, -p.y),
    |p| Point::new(-p.z, p.y, p.x),
    |p| Point::new(-p.z, -p.y, -p.x),
];

/// Enumerates all transforms such that
///  f(b) = a, and f applies one of the 24 rotational symmetries of a cube
fn enumerate_transforms(
    a: Point,
    b: Point,
) -> impl Iterator<Item = impl Fn(Point) -> Point + Clone> {
    ROTATIONS.iter().map(move |r| {
        let offset = a - r(b);
        move |p| r(p) + offset
    })
}

/// Find a stations and it's transform which can be used to add becons
/// to the joined set
fn find_transform(
    stations: &[(Vec<Point>, HashMap<i32, usize>)],
    joined_becons: &HashSet<Point>,
) -> Option<(usize, impl Fn(Point) -> Point + Clone)> {
    // Compute every combination of distances between joined points
    let joined_dist_freq = joined_becons
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a - b).l2_squared_norm())
        .counts();

    for (i, (station, station_freq)) in stations.iter().enumerate() {
        // Check if the distances frequency overlaps with this
        // station. If it doesn't we can quickly skip this station
        // because it cannot possibly have enough overlapping beacons
        // after transforming.
        let mut overlap = 0;
        for (dist, freq) in station_freq {
            overlap += freq.min(joined_dist_freq.get(dist).unwrap_or(&0));
        }

        if overlap < 12 * (12 - 1) / 2 {
            continue;
        }

        // Test every point in the station against the joined
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
                        >= 12
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
    // Stations still left to process + their distance frequencies
    let mut stations: Vec<(_, _)> = parse_input(input)
        .map(|s| {
            let freqs = s
                .iter()
                .tuple_combinations()
                .map(|(a, b)| (a - b).l2_squared_norm())
                .counts();
            (s, freqs)
        })
        .collect();
    let mut station_centers = Vec::with_capacity(stations.len());

    // Becons part of the final becon set (initially station 0)
    let mut joined_becons: HashSet<Point> = stations.swap_remove(0).0.into_iter().collect();

    while !stations.is_empty() {
        if let Some((station, trans)) = find_transform(&stations, &joined_becons) {
            // Rotate the stations so that stations we already
            // searched but couldn't join are not searched again
            // immediately. This line results in about 2x performance
            // on my test input.
            stations.rotate_left(station + 1);

            // Move station into the joined set using the found transform.
            joined_becons.extend(stations.pop().unwrap().0.into_iter().map(trans.clone()));
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
