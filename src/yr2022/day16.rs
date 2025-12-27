use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use nalgebra::DMatrix;

#[derive(Clone, Debug, Default)]
struct Valve {
    rate: u32,
    outgoing: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Valve> {
    // Parse into a map based structure first
    let map: HashMap<&str, (u32, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split([' ', '=']).collect();
            (
                parts[1],
                (
                    parts[5].trim_matches(';').parse().unwrap(),
                    parts[10..].iter().map(|p| p.trim_matches(',')).collect(),
                ),
            )
        })
        .collect();

    // Rewrite map using zero based keys
    let index_to_id: Vec<&str> = map.keys().copied().sorted().collect();
    index_to_id
        .iter()
        .map(|id| {
            let (rate, outgoing) = &map[id];
            Valve {
                rate: *rate,
                outgoing: outgoing
                    .iter()
                    .map(|oid| index_to_id.iter().position(|s| s == oid).unwrap())
                    .collect(),
            }
        })
        .collect()
}

/// Takes a list of values and runs pathfinding to reduce the problem
/// to a list of non-zero flow rates, and a path distance matrix
fn pathfinding(valves: &[Valve]) -> (Vec<u32>, DMatrix<u32>) {
    let pfid_to_vid: Vec<_> = valves
        .iter()
        .enumerate()
        .filter(|&(i, v)| i == 0 || v.rate > 0)
        .map(|(i, _)| i)
        .collect();
    let mut costs = DMatrix::repeat(pfid_to_vid.len(), pfid_to_vid.len(), u32::MAX);

    for (from_pfid, &from_vid) in pfid_to_vid.iter().enumerate() {
        let mut open = VecDeque::new();
        let mut closed = vec![false; valves.len()];

        open.push_back((0, from_vid));

        while let Some((dist, node_vid)) = open.pop_front() {
            if !closed[node_vid] {
                closed[node_vid] = true;

                if let Some(node_pfid) = pfid_to_vid.iter().position(|&v| v == node_vid) {
                    costs[(from_pfid, node_pfid)] = dist;
                }

                for &child_vid in valves[node_vid].outgoing.iter() {
                    open.push_back((dist + 1, child_vid));
                }
            }
        }
    }

    assert!(costs.iter().all(|&c| c < u32::MAX));
    (
        pfid_to_vid
            .into_iter()
            .map(|vid| valves[vid].rate)
            .collect(),
        costs,
    )
}

/// Runs a DFS with the given start point and returns the best pressure release
///  Will not attempt to open valve at "start". Score does not include opened valves.
fn dfs(
    rates: &[u32],
    costs: &DMatrix<u32>,
    opened: &mut Vec<usize>,
    start: usize,
    time_left: u32,
) -> u32 {
    let mut best = 0;

    if time_left > 1 {
        for (vid, &rate) in rates.iter().enumerate() {
            if vid != 0
                && vid != start
                && let Some(new_time_left) = time_left.checked_sub(costs[(start, vid)] + 1)
                && new_time_left > 0
                && !opened.contains(&vid)
            {
                opened.push(vid);
                best =
                    best.max(rate * new_time_left + dfs(rates, costs, opened, vid, new_time_left));
                opened.pop();
            }
        }
    }

    best
}

pub fn star1(input: &str) -> String {
    let (rates, costs) = pathfinding(&parse_input(input));
    dfs(&rates, &costs, &mut Vec::new(), 0, 30).to_string()
}

fn generate_opened(opened: &mut Vec<usize>, rates_len: usize, i: usize) {
    opened.clear();
    for j in 0..(rates_len - 1) {
        if i & (1 << j) != 0 {
            opened.push(j + 1);
        }
    }
}

pub fn star2(input: &str) -> String {
    let (rates, costs) = pathfinding(&parse_input(input));
    let mut opened = Vec::with_capacity(rates.len());
    let mut best = 0;

    for i in 0..(1 << (rates.len() - 2)) {
        generate_opened(&mut opened, rates.len(), i);
        let score_a = dfs(&rates, &costs, &mut opened, 0, 26);

        generate_opened(&mut opened, rates.len(), !i);
        let upper_bound = rates
            .iter()
            .enumerate()
            .filter(|(i, _)| !opened.contains(i))
            .map(|(_, r)| r)
            .sum::<u32>()
            * 24;
        if score_a + upper_bound > best {
            let score_b = dfs(&rates, &costs, &mut opened, 0, 26);
            best = best.max(score_a + score_b);
        }
    }

    best.to_string()
}
