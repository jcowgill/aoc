use std::collections::{HashMap, HashSet};

fn trim_bag(mut bag: &str) -> (u32, &str) {
    bag = bag.trim_start();
    let number = if bag.starts_with(|c: char| c.is_ascii_digit()) {
        let (a, b) = bag.split_once(' ').unwrap();
        bag = b;
        a.parse().unwrap()
    } else {
        0
    };

    (
        number,
        bag.trim_end_matches(&['.', ' '])
            .trim_end_matches("bags")
            .trim_end_matches("bag")
            .trim(),
    )
}

fn parse_line(line: &str) -> (&str, impl Iterator<Item = (u32, &str)>) {
    let (l, r) = line.split_once("contain").unwrap();
    (
        trim_bag(l).1,
        r.split(',').map(trim_bag).filter(|&(_, b)| b != "no other"),
    )
}

pub fn star1(input: &str) -> String {
    let mut within = HashMap::new();
    for (bag, contain) in input.lines().map(parse_line) {
        for (_, c) in contain {
            within.entry(c).or_insert(Vec::new()).push(bag);
        }
    }

    let mut seen = HashSet::new();
    let mut open = vec!["shiny gold"];
    while let Some(bag) = open.pop() {
        if seen.insert(bag) {
            if let Some(w) = within.get(bag) {
                open.extend_from_slice(w);
            }
        }
    }

    (seen.len() - 1).to_string()
}

pub fn star2(input: &str) -> String {
    let contains: HashMap<_, Vec<_>> = input
        .lines()
        .map(|l| {
            let (bag, it) = parse_line(l);
            (bag, it.collect())
        })
        .collect();
    let mut total = 0;
    let mut open = HashMap::new();
    open.insert("shiny gold", 1);

    while let Some((&b, &n)) = open.iter().next() {
        open.remove(b);
        total += n;
        for (m, c) in contains[b].iter() {
            *open.entry(c).or_insert(0) += n * m;
        }
    }

    (total - 1).to_string()
}
