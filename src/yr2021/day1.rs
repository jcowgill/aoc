use itertools::Itertools;

pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|&(a, b)| b > a)
        .count()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|&(a, b)| b > a)
        .count()
        .to_string()
}
