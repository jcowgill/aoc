use itertools::Itertools;

fn parse_input(input: &str) -> impl Iterator<Item = (i32, i32, i32, i32)> + '_ {
    input.lines().map(|line| {
        line.trim()
            .split(['-', ','])
            .map(|v| v.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

pub fn star1(input: &str) -> String {
    parse_input(input)
        .filter(|(a1, a2, b1, b2)| {
            let c1 = a1.max(b1);
            let c2 = a2.min(b2);
            (c1 == a1 && c2 == a2) || (c1 == b1 && c2 == b2)
        })
        .count()
        .to_string()
}

pub fn star2(input: &str) -> String {
    parse_input(input)
        .filter(|(a1, a2, b1, b2)| a1.max(b1) <= a2.min(b2))
        .count()
        .to_string()
}
