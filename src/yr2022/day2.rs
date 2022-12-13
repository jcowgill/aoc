pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let a = l.as_bytes()[0] - b'A';
            let b = l.as_bytes()[2] - b'X';
            let outcome = if a == b {
                3
            } else if (a + 1) % 3 == b {
                6
            } else {
                0
            };

            u32::from(outcome + b + 1)
        })
        .sum::<u32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let a = l.as_bytes()[0] - b'A';
            u32::from(match l.as_bytes()[2] {
                b'X' => 1 + (a + 2) % 3,
                b'Y' => 4 + a,
                b'Z' => 7 + (a + 1) % 3,
                _ => panic!("invalid input"),
            })
        })
        .sum::<u32>()
        .to_string()
}
