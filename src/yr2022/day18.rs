use itertools::Itertools;
use nalgebra::Vector3;
use std::collections::HashSet;

const DIRECTIONS: [Vector3<i32>; 6] = [
    Vector3::new(1, 0, 0),
    Vector3::new(0, 1, 0),
    Vector3::new(0, 0, 1),
    Vector3::new(-1, 0, 0),
    Vector3::new(0, -1, 0),
    Vector3::new(0, 0, -1),
];

fn parse_input(input: &str) -> HashSet<Vector3<i32>> {
    input
        .lines()
        .map(|l| Vector3::from_iterator(l.split(',').map(|p| p.parse().unwrap())))
        .collect()
}

fn total_sides(cubes: &HashSet<Vector3<i32>>) -> usize {
    let adjacent = cubes
        .iter()
        .cartesian_product(DIRECTIONS)
        .map(|(cube, side)| cubes.contains(&(cube + side)) as usize)
        .sum::<usize>();

    cubes.len() * 6 - adjacent
}

pub fn star1(input: &str) -> String {
    total_sides(&parse_input(input)).to_string()
}

fn dfs(seen: &mut HashSet<Vector3<i32>>, min: i32, max: i32) {
    let mut open = Vec::new();
    open.push(Vector3::repeat(min));

    while let Some(pos) = open.pop() {
        if !seen.contains(&pos) && pos.iter().all(|c| (min..=max).contains(c)) {
            seen.insert(pos);
            for dir in DIRECTIONS {
                open.push(pos + dir);
            }
        }
    }
}

pub fn star2(input: &str) -> String {
    let mut cubes = parse_input(input);
    let all_sides = total_sides(&cubes);

    let min = cubes.iter().map(|v| v.min()).min().unwrap();
    let max = cubes.iter().map(|v| v.max()).max().unwrap();
    dfs(&mut cubes, min, max);

    let inverse_cubes = (min..=max)
        .cartesian_product(min..=max)
        .cartesian_product(min..=max)
        .map(|((x, y), z)| Vector3::new(x, y, z))
        .filter(|v| !cubes.contains(v))
        .collect();
    let bad_sides = total_sides(&inverse_cubes);

    (all_sides - bad_sides).to_string()
}
