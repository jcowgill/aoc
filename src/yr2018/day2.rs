use std::collections::HashMap;
use std::hash::Hash;

/// Returns map containing the frequencies of each item in the given
/// iterator
fn frequency_map<I, T>(iter: I) -> HashMap<T, usize>
    where I: Iterator<Item=T>, T: Eq + Hash {

    let mut map = HashMap::new();
    for item in iter {
        *map.entry(item).or_insert(0) += 1;
    }
    map
}

/// Returns checksum of input box ids
pub fn star1(input: &str) -> String {
    let mut exactly_twice = 0;
    let mut exactly_thrice = 0;

    for freqs in input.lines().map(|line| frequency_map(line.trim().chars())) {
        if freqs.values().any(|&f| f == 2) { exactly_twice += 1; }
        if freqs.values().any(|&f| f == 3) { exactly_thrice += 1; }
    }

    (exactly_twice * exactly_thrice).to_string()
}

/// Returns the Hamming distance of two strings
fn hamming_distance(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());
    a.chars().zip(b.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

/// Returns a new string containing the characters common to both
fn hamming_common(a: &str, b: &str) -> String {
    assert_eq!(a.len(), b.len());
    a.chars().zip(b.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c, _)| c)
        .collect()
}

/// Finds the first pair of lines with hamming distance 1 and returns
/// common chars
pub fn star2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();

    for i in 0..lines.len() {
        for j in 0..i {
            if hamming_distance(lines[i], lines[j]) == 1 {
                return hamming_common(lines[i], lines[j])
            }
        }
    }

    panic!("Prototype fabric not found!")
}
