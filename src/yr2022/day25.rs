fn from_snafu(num: &str) -> i64 {
    num.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("invalid character {}", c),
            }
    })
}

fn to_snafu(num: i64) -> String {
    match num % 5 {
        0 if num == 0 => String::new(),
        0 => to_snafu(num / 5) + "0",
        1 => to_snafu(num / 5) + "1",
        2 => to_snafu(num / 5) + "2",
        3 => to_snafu(num / 5 + 1) + "=",
        4 => to_snafu(num / 5 + 1) + "-",
        _ => panic!("invalid number"),
    }
}

pub fn star1(input: &str) -> String {
    to_snafu(input.lines().map(from_snafu).sum::<i64>())
}
