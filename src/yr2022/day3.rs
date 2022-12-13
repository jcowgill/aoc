use std::collections::HashSet;

use itertools::Itertools;

fn priority(item: u8) -> u32 {
    if item >= b'a' {
        u32::from(item - b'a' + 1)
    } else {
        u32::from(item - b'A' + 27)
    }
}

pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            let line_bytes = l.trim().as_bytes();
            let (a, b) = line_bytes.split_at(line_bytes.len() / 2);

            let a_set: HashSet<_> = a.iter().copied().collect();
            priority(*b.iter().find(|&c| a_set.contains(c)).unwrap())
        })
        .sum::<u32>()
        .to_string()
}

fn line_to_set(line: &str) -> HashSet<u8> {
    line.trim().as_bytes().iter().copied().collect()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let mut set: HashSet<_> = line_to_set(chunk.next().unwrap());

            for line in chunk {
                set = line_to_set(line).intersection(&set).copied().collect();
            }

            priority(set.into_iter().exactly_one().unwrap())
        })
        .sum::<u32>()
        .to_string()
}
