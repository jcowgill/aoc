use std::cmp::Ordering;

use crate::vector::VectorExt;
use nalgebra::Vector2;

/// Parses a single coordinate line
fn parse_coord(line: &str) -> Vector2<i32> {
    let coords: Vec<i32> = line
        .splitn(2, ',')
        .map(|p| p.trim().parse().unwrap())
        .collect();

    Vector2::from_row_slice(&coords)
}

/// Find the bounding rectangle (as x1y1, x2y2) of a set of points
fn find_bounding_rect<'a, I>(points: I) -> Option<(Vector2<i32>, Vector2<i32>)>
where
    I: Iterator<Item = &'a Vector2<i32>> + Clone,
{
    Some((
        Vector2::new(
            points.clone().map(|v| v.x).min()?,
            points.clone().map(|v| v.y).min()?,
        ),
        Vector2::new(
            points.clone().map(|v| v.x).max()?,
            points.map(|v| v.y).max()?,
        ),
    ))
}

/// Find the nearest coord id to a given point
///  Returns None if there is no nearest point (ie no coords, or 2+ equal coords)
fn find_nearest_coord<'a, I>(point: Vector2<i32>, coords: I) -> Option<usize>
where
    I: Iterator<Item = &'a Vector2<i32>>,
{
    let mut nearest_id = None;
    let mut nearest_dist = std::i32::MAX;

    for (id, &coord) in coords.enumerate() {
        let dist = (point - coord).taxicab_norm();

        match dist.cmp(&nearest_dist) {
            Ordering::Less => {
                nearest_id = Some(id);
                nearest_dist = dist;
            }
            Ordering::Equal => nearest_id = None,
            Ordering::Greater => (),
        }
    }

    nearest_id
}

/// Find largest area surrounding each coordinate of a set
pub fn star1(input: &str) -> String {
    let coords: Vec<Vector2<i32>> = input.lines().map(parse_coord).collect();
    let bounds = find_bounding_rect(coords.iter()).unwrap();

    let mut areas: Vec<Option<usize>> = Vec::new();
    areas.resize(coords.len(), Some(0));

    // Traverse entire rectangle finding the nearest coords
    for x in bounds.0.x..=bounds.1.x {
        for y in bounds.0.y..=bounds.1.y {
            if let Some(id) = find_nearest_coord(Vector2::new(x, y), coords.iter()) {
                // If this coordinate is on the boundary, the area found is infinite
                if x == bounds.0.x || x == bounds.1.x || y == bounds.0.y || y == bounds.1.y {
                    areas[id] = None;
                } else {
                    areas[id] = areas[id].map(|s| s + 1);
                }
            }
        }
    }

    areas.iter().filter_map(|&s| s).max().unwrap().to_string()
}

/// Find size of area "close" to a set of all points
pub fn star2(input: &str) -> String {
    let mut input_lines = input.lines().peekable();
    let safe_distance: i32 = if input_lines.peek().map_or(false, |l| !l.contains(',')) {
        input_lines.next().unwrap().parse().unwrap()
    } else {
        10000
    };

    let coords: Vec<Vector2<i32>> = input_lines.map(parse_coord).collect();
    let bounds = find_bounding_rect(coords.iter()).unwrap();
    let mut safe_zone_size = 0;

    for x in bounds.0.x..=bounds.1.x {
        for y in bounds.0.y..=bounds.1.y {
            let current = Vector2::new(x, y);
            let dist_sum: i32 = coords.iter().map(|&c| (current - c).taxicab_norm()).sum();

            if dist_sum < safe_distance {
                safe_zone_size += 1;
            }
        }
    }

    safe_zone_size.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1A, "17");
    star_test!(me1, star1, ME, "2906");

    star_test!(example1b, star2, IN1B, "16");
    star_test!(me2, star2, ME, "50530");

    const IN1A: &str = indoc! {"
        1, 1
        1, 6
        8, 3
        3, 4
        5, 5
        8, 9
    "};

    const IN1B: &str = indoc! {"
        32
        1, 1
        1, 6
        8, 3
        3, 4
        5, 5
        8, 9
    "};

    const ME: &str = indoc! {"
        227, 133
        140, 168
        99, 112
        318, 95
        219, 266
        134, 144
        306, 301
        189, 188
        58, 334
        337, 117
        255, 73
        245, 144
        102, 257
        255, 353
        303, 216
        141, 167
        40, 321
        201, 50
        60, 188
        132, 74
        125, 199
        176, 307
        204, 218
        338, 323
        276, 278
        292, 229
        109, 228
        85, 305
        86, 343
        97, 254
        182, 151
        110, 292
        285, 124
        43, 223
        153, 188
        285, 136
        334, 203
        84, 243
        92, 185
        330, 223
        259, 275
        106, 199
        183, 205
        188, 212
        231, 150
        158, 95
        174, 212
        279, 97
        172, 131
        247, 320
    "};
}
