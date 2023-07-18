fn seats(input: &str) -> impl Iterator<Item = u16> + '_ {
    input.lines().map(|l| {
        l.chars()
            .fold(0, |acc, c| acc << 1 | (c == 'B' || c == 'R') as u16)
    })
}

pub fn star1(input: &str) -> String {
    seats(input).max().unwrap().to_string()
}

pub fn star2(input: &str) -> String {
    let mut v: Vec<u16> = seats(input).collect();
    v.sort_unstable();
    let min = (*v.first().unwrap() | 7) + 1;
    let max = *v.last().unwrap() & !7;
    v.into_iter()
        .filter(|id| (min..max).contains(id))
        .zip(min..max)
        .find(|(a, b)| a != b)
        .unwrap()
        .1
        .to_string()
}
