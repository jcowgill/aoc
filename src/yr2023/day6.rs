use itertools::{multizip, Itertools};

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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1, star1, IN1, "288");
    star_test!(me1, star1, ME, "449820");

    star_test!(example2, star2, IN1, "71503");
    star_test!(me2, star2, ME, "42250895");

    const IN1: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    const ME: &str = indoc! {"
        Time:        53     71     78     80
        Distance:   275   1181   1215   1524
    "};
}
