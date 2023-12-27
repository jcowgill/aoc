use nalgebra::Vector2;

use crate::vector::VectorExt;

fn parse_input(input: &str) -> Vec<Vector2<i64>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| Vector2::new(x as i64, y as i64))
        })
        .collect()
}

fn offsets(galaxies: &[Vector2<i64>], f: impl Fn(&Vector2<i64>) -> i64) -> Vec<i64> {
    let max = galaxies.iter().map(&f).max().unwrap() as usize;
    let mut result = vec![1; max + 1];
    for g in galaxies {
        result[f(g) as usize] = 0;
    }
    for i in 1..result.len() {
        result[i] += result[i - 1];
    }
    result
}

fn solve(input: &str, growth: i64) -> String {
    let mut galaxies = parse_input(input);
    let xoffsets = offsets(&galaxies, |g| g.x);
    let yoffsets = offsets(&galaxies, |g| g.y);

    for g in &mut galaxies {
        g.x += xoffsets[g.x as usize] * (growth - 1);
        g.y += yoffsets[g.y as usize] * (growth - 1);
    }

    (0..galaxies.len())
        .map(|i| {
            (0..i)
                .map(|j| (galaxies[i] - galaxies[j]).taxicab_norm())
                .sum::<i64>()
        })
        .sum::<i64>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    solve(input, 2)
}

pub fn star2(input: &str) -> String {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve10() {
        assert_eq!(solve(IN1, 10), "1030");
    }

    #[test]
    fn solve100() {
        assert_eq!(solve(IN1, 100), "8410");
    }

    const IN1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
}
