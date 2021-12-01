use lazy_static::lazy_static;
use regex::Regex;

use crate::vector::Vector2;

/// Returns the smallest rectangle which bounds a set of points
fn bounding_rect<'a, I: Iterator<Item = &'a Vector2<i32>>>(
    mut points: I,
) -> Option<(Vector2<i32>, Vector2<i32>)> {
    if let Some(&initial) = points.next() {
        let (mut p1, mut p2) = (initial, initial);

        for point in points {
            if point.x < p1.x {
                p1.x = point.x;
            } else if point.x > p2.x {
                p2.x = point.x;
            }

            if point.y < p1.y {
                p1.y = point.y;
            } else if point.y > p2.y {
                p2.y = point.y;
            }
        }

        Some((p1, p2))
    } else {
        None
    }
}

/// Returns the size of the smallest rectangle which bounds a set of points
fn bounding_rect_size<'a, I: Iterator<Item = &'a Vector2<i32>>>(points: I) -> Option<i32> {
    bounding_rect(points).map(|(p1, p2)| (p2 - p1).taxicab_norm())
}

/// Finds the smallest bounding rectangle of a set of points by scanning through time
///  Returns the number of iterations taken.
///  The points vector contains the best points
fn bounding_rect_smallest(points: &mut Vec<Vector2<i32>>, velocities: &[Vector2<i32>]) -> usize {
    assert_eq!(points.len(), velocities.len());

    let mut prev_score = i32::max_value();
    let mut score = bounding_rect_size(points.iter()).unwrap();
    let mut iterations = 0;

    // Keep adding velocities until we get a higher score than before
    while score < prev_score {
        for (point, velocity) in points.iter_mut().zip(velocities.iter()) {
            *point += *velocity;
        }

        prev_score = score;
        score = bounding_rect_size(points.iter()).unwrap();
        iterations += 1;
    }

    // Go back one iteration to get the best points
    for (point, velocity) in points.iter_mut().zip(velocities.iter()) {
        *point -= *velocity;
    }

    iterations - 1
}

/// Renders a set of points to a string
fn points_to_string(mut points: Vec<Vector2<i32>>) -> String {
    let mut result = String::new();

    if !points.is_empty() {
        // Sort input points into the order to be displayed
        points.sort_unstable_by(|&a, &b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        points.dedup();

        // Render each point after inserting needed passing
        let (mut cursor, _) = bounding_rect(points.iter()).unwrap();
        for point in points {
            for _ in cursor.y..point.y {
                result.push('\n')
            }
            for _ in cursor.x..point.x {
                result.push(' ')
            }
            result.push('#');
            cursor = point + Vector2 { x: 1, y: 0 };
        }
    }

    result
}

/// Parses the input into point and velocity vectors
fn parse_input(input: &str) -> (Vec<Vector2<i32>>, Vec<Vector2<i32>>) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^position=<\s*([0-9-]+)\s*,\s*([0-9-]+)\s*>\s*velocity=<\s*([0-9-]+)\s*,\s*([0-9-]+)\s*>$")
            .unwrap();
    }

    let mut points: Vec<Vector2<i32>> = Vec::new();
    let mut velocities: Vec<Vector2<i32>> = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = RE
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse().unwrap())
            .collect();

        points.push(Vector2 {
            x: parts[0],
            y: parts[1],
        });
        velocities.push(Vector2 {
            x: parts[2],
            y: parts[3],
        });
    }

    (points, velocities)
}

pub fn star1(input: &str) -> String {
    let (mut points, velocities) = parse_input(input);
    bounding_rect_smallest(&mut points, &velocities);
    points_to_string(points)
}

pub fn star2(input: &str) -> String {
    let (mut points, velocities) = parse_input(input);
    bounding_rect_smallest(&mut points, &velocities).to_string()
}
