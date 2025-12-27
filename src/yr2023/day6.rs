use itertools::{Itertools, multizip};

fn solve((t, d): (i64, i64)) -> i64 {
    // y = x * (t - x) - d = xt - x^2 - (d + 1)
    // Count number of integer x such that y >= 0
    let sqr_d2 = ((t * t - 4 * (d + 1)) as f64).sqrt() / 2.0;
    let min = (t as f64 / 2.0 - sqr_d2).ceil() as i64;
    let max = (t as f64 / 2.0 + sqr_d2).floor() as i64;
    max - min + 1
}

pub fn star1(input: &str) -> String {
    multizip(
        input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .skip(1)
                    .map(|n| n.parse::<i64>().unwrap())
            })
            .collect_tuple::<(_, _)>()
            .unwrap(),
    )
    .map(solve)
    .product::<i64>()
    .to_string()
}

pub fn star2(input: &str) -> String {
    solve(
        input
            .lines()
            .map(|line| {
                line.split_once(':')
                    .unwrap()
                    .1
                    .chars()
                    .filter(char::is_ascii_digit)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap(),
    )
    .to_string()
}
