use std::collections::HashMap;

use itertools::Itertools;

type Pair = (u8, u8);
type FreqMap = HashMap<u8, usize>;

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

/// Adds the values from frequency map b into map a
fn add_freq_map(a: &mut FreqMap, b: &FreqMap) {
    for (&c, freq) in b.iter() {
        *a.entry(c).or_default() += freq
    }
}

fn count_freqs(
    mappings: &HashMap<Pair, u8>,
    cache: &mut HashMap<(Pair, usize), HashMap<u8, usize>>,
    pair: Pair,
    depth: usize,
) {
    if !cache.contains_key(&(pair, depth)) {
        let mut map;

        // If we reached the bottom generate a new map with just this pair
        if depth == 0 {
            map = HashMap::new();

            if pair.0 == pair.1 {
                map.insert(pair.0, 2);
            } else {
                map.insert(pair.0, 1);
                map.insert(pair.1, 1);
            }
        } else {
            // Expand pair and count frequencies on each side
            let middle = mappings[&pair];
            let left_pair = (pair.0, middle);
            let right_pair = (middle, pair.1);
            count_freqs(mappings, cache, left_pair, depth - 1);
            count_freqs(mappings, cache, right_pair, depth - 1);

            // Create new frequency map by adding up sub pairs and
            // subtracting duplicate middle value
            map = cache[&(left_pair, depth - 1)].clone();
            add_freq_map(&mut map, &cache[&(right_pair, depth - 1)]);
            *map.get_mut(&middle).unwrap() -= 1;
        }

        cache.insert((pair, depth), map);
    }
}

fn star_common(input: &str, depth: usize) -> String {
    let (start_chars, mappings) = parse_input(input);
    let mut cache = HashMap::new();
    let mut map =
        start_chars
            .iter()
            .copied()
            .tuple_windows()
            .fold(HashMap::new(), |mut map, pair| {
                count_freqs(&mappings, &mut cache, pair, depth);
                add_freq_map(&mut map, &cache[&(pair, depth)]);
                map
            });

    // Sutract all middle characters from start string
    (1..(start_chars.len() - 1)).for_each(|i| {
        *map.get_mut(&start_chars[i]).unwrap() -= 1;
    });

    let (min, max) = map.into_values().minmax().into_option().unwrap();
    (max - min).to_string()
}

pub fn star1(input: &str) -> String {
    star_common(input, 10)
}

pub fn star2(input: &str) -> String {
    star_common(input, 40)
}
