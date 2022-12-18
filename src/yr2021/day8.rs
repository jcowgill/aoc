use itertools::Itertools;

static NUMBERS: [[u8; 7]; 10] = [
    [b'a', b'b', b'c', b'e', b'f', b'g', 0],
    [b'c', b'f', 0, 0, 0, 0, 0],
    [b'a', b'c', b'd', b'e', b'g', 0, 0],
    [b'a', b'c', b'd', b'f', b'g', 0, 0],
    [b'b', b'c', b'd', b'f', 0, 0, 0],
    [b'a', b'b', b'd', b'f', b'g', 0, 0],
    [b'a', b'b', b'd', b'e', b'f', b'g', 0],
    [b'a', b'c', b'f', 0, 0, 0, 0],
    [b'a', b'b', b'c', b'd', b'e', b'f', b'g'],
    [b'a', b'b', b'c', b'd', b'f', b'g', 0],
];

/// Deduces the display values for a possible valid wire combination
fn deduce_values(line: &str) -> Vec<u32> {
    let (left, right): (Vec<_>, Vec<_>) = {
        let (left_str, right_str) = line.split_once('|').unwrap();
        (
            left_str.split_ascii_whitespace().collect(),
            right_str.split_ascii_whitespace().collect(),
        )
    };

    // Go through every permutation of wires
    //  Each permutation is a mapping into the "correct" wire positions
    for perm in (b'a'..=b'g').permutations(7) {
        let map_str = |&s: &&str| -> Vec<_> {
            s.as_bytes()
                .iter()
                .map(|b| perm[(b - b'a') as usize])
                .sorted()
                .chain([0; 7])
                .take(7)
                .collect()
        };

        // Test all the numbers in the "signal patterns" section
        if left
            .iter()
            .map(map_str)
            .all(|s| NUMBERS.iter().any(|n| n[..] == s))
        {
            // Find numbers in the "output" section
            return right
                .iter()
                .map(|s| NUMBERS.iter().position(|n| n[..] == map_str(s)).unwrap() as u32)
                .collect();
        }
    }

    panic!("no deduction found for line '{line}'")
}

pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            deduce_values(l)
                .iter()
                .filter(|v| [1, 4, 7, 8].contains(v))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .map(|l| deduce_values(l).iter().fold(0, |acc, d| acc * 10 + d))
        .sum::<u32>()
        .to_string()
}
