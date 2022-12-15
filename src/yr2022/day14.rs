use std::collections::HashSet;

use itertools::Itertools;

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, i32) {
    let mut result = HashSet::new();
    let mut max_y = i32::MIN;

    for line in input.lines() {
        for ((a1, a2), (b1, b2)) in line
            .split("->")
            .map(|p| {
                p.split(',')
                    .map(|n| n.trim().parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .tuple_windows()
        {
            for x in a1.min(b1)..=a1.max(b1) {
                for y in a2.min(b2)..=a2.max(b2) {
                    result.insert((x, y));
                }
            }

            max_y = max_y.max(a2.max(b2));
        }
    }

    (result, max_y)
}

/// Simulates falling sand. Returns true if the sand came to rest.
fn sand(map: &mut HashSet<(i32, i32)>, abyss: i32, x: i32, y: i32) -> bool {
    if y > abyss {
        false
    } else {
        if map.insert((x, y)) {
            for ox in [0, -1, 1] {
                if !sand(map, abyss, x + ox, y + 1) {
                    map.remove(&(x, y));
                    return false;
                }
            }
        }
        true
    }
}

fn count_sand(mut map: HashSet<(i32, i32)>, abyss: i32) -> usize {
    let start = map.len();
    sand(&mut map, abyss, 500, 0);
    map.len() - start
}

pub fn star1(input: &str) -> String {
    let (map, max_y) = parse_input(input);
    count_sand(map, max_y).to_string()
}

pub fn star2(input: &str) -> String {
    let (mut map, max_y) = parse_input(input);
    for x in 0..1000 {
        map.insert((x, max_y + 2));
    }
    count_sand(map, max_y + 2).to_string()
}
