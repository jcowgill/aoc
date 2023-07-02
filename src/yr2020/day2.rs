fn parse_line(line: &str) -> (usize, usize, char, &str) {
    let parts: Vec<_> = line.split(&[' ', ':', '-']).collect();
    (
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap(),
        parts[2].chars().next().unwrap(),
        parts[4],
    )
}

pub fn star1(input: &str) -> String {
    input
        .lines()
        .filter(|&line| {
            let (min, max, ch, s) = parse_line(line);
            let count = s.chars().filter(|&c| c == ch).count();
            (min..=max).contains(&count)
        })
        .count()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .filter(|&line| {
            let (a, b, ch, s) = parse_line(line);
            let count = (s.chars().nth(a - 1) == Some(ch)) as i32
                + (s.chars().nth(b - 1) == Some(ch)) as i32;
            count == 1
        })
        .count()
        .to_string()
}
