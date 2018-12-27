use std::collections::HashMap;
use std::str::FromStr;

use crate::vector::Vector2;

/// A 2D rectangle with integer dimensions and an ID
#[derive(Debug, Clone)]
struct Rectangle
{
    id: u32,
    pos: Vector2<u32>,
    size: Vector2<u32>,
}

impl FromStr for Rectangle
{
    type Err = ();

    fn from_str(s: &str) -> Result<Rectangle, ()> {
        let parts: Vec<Result<u32, ()>> = s
            .trim_start_matches('#')
            .split(|c| ['@', ',', ':', 'x'].contains(&c))
            .map(|part| part.trim().parse().map_err(|_| ()))
            .collect();

        if parts.len() == 5 && parts.iter().all(|part| part.is_ok()) {
            return Ok(Rectangle {
                id: parts[0]?,
                pos: Vector2 { x: parts[1]?, y: parts[2]? },
                size: Vector2 { x: parts[3]?, y: parts[4]? },
            });
        }

        Err(())
    }
}

/// Returns a hash map mapping points to the number of times they
/// appear in the rectangle iterator given
fn count_points_in_rectangles<I>(iter: I) -> HashMap<Vector2<u32>, usize>
    where I: Iterator<Item=Rectangle> {

    let mut result = HashMap::new();

    for rect in iter {
        for y in 0..rect.size.y {
            for x in 0..rect.size.x {
                *result.entry(Vector2 { x: x, y: y } + rect.pos).or_insert(0) += 1;
            }
        }
    }


    result
}

/// Parses the input rectangles string into an iterator
fn parse_rectangles<'a>(input: &'a str) -> impl Iterator<Item=Rectangle> + Clone + 'a {
    input.lines().map(|line| line.parse::<Rectangle>().unwrap())
}

/// Find the number of overlapping points in the input rectangles
pub fn star1(input: &str) -> String {
    count_points_in_rectangles(parse_rectangles(input))
        .values()
        .filter(|&v| v >= &2)
        .count()
        .to_string()
}

/// Find the only rectangle which does not overlap with any other
pub fn star2(input: &str) -> String {
    let rect_iter = parse_rectangles(input);
    let points_map = count_points_in_rectangles(rect_iter.clone());

    // Search the rectangles again for a rectangle whose points only
    // contain 1s in the point map
    for rect in rect_iter {
        let mut non_overlapping = true;

        for y in 0..rect.size.y {
            for x in 0..rect.size.x {
                if points_map[&(Vector2 { x: x, y: y } + rect.pos)] != 1 {
                    non_overlapping = false;
                }
            }
        }

        if non_overlapping {
            return rect.id.to_string();
        }
    }

    panic!("no non-overlapping rectangle?!");
}
