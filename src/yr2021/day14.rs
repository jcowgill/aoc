use std::collections::HashMap;

use itertools::Itertools;

type Pair = (u8, u8);

fn parse_input(input: &str) -> (Vec<u8>, HashMap<Pair, u8>) {
    let mut lines = input.lines();
    let start_chars = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c as u8)
        .collect();
    let mappings = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let chars: Vec<u8> = l
                .chars()
                .filter(char::is_ascii_alphabetic)
                .map(|c| c as u8)
                .collect();
            ((chars[0], chars[1]), chars[2])
        })
        .collect();

    (start_chars, mappings)
}

fn star_common(input: &str, depth: usize) -> String {
    let (start_chars, mappings) = parse_input(input);
    let mut char_freqs = start_chars.iter().copied().counts();
    let mut pair_freqs = start_chars.into_iter().tuple_windows().counts();

    for _ in 0..depth {
        for (pair, freq) in pair_freqs
            .iter()
            .map(|(&pair, &freq)| (pair, freq))
            .collect_vec()
        {
            let middle = mappings[&pair];
            *pair_freqs.get_mut(&pair).unwrap() -= freq;
            *pair_freqs.entry((pair.0, middle)).or_default() += freq;
            *pair_freqs.entry((middle, pair.1)).or_default() += freq;
            *char_freqs.entry(middle).or_default() += freq;
        }
    }

    let (min, max) = char_freqs.into_values().minmax().into_option().unwrap();
    (max - min).to_string()
}

pub fn star1(input: &str) -> String {
    star_common(input, 10)
}

pub fn star2(input: &str) -> String {
    star_common(input, 40)
}
