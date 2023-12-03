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
            if vid != 0 && vid != start {
                if let Some(new_time_left) = time_left.checked_sub(costs[(start, vid)] + 1) {
                    if new_time_left > 0 && !opened.contains(&vid) {
                        opened.push(vid);
                        best = best.max(
                            rate * new_time_left + dfs(rates, costs, opened, vid, new_time_left),
                        );
                        opened.pop();
                    }
                }
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "1651");
    star_test!(me1, star1, ME, "1871");

    star_test!(example1b, star2, IN1, "1707");
    star_test!(me2, star2, ME, "2416");

    const IN1: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    const ME: &str = indoc! {"
        Valve OJ has flow rate=0; tunnels lead to valves EW, IG
        Valve BN has flow rate=0; tunnels lead to valves SA, AA
        Valve SA has flow rate=5; tunnels lead to valves QK, LP, ZP, BN, VH
        Valve RL has flow rate=21; tunnel leads to valve AM
        Valve LR has flow rate=19; tunnel leads to valve XZ
        Valve VQ has flow rate=0; tunnels lead to valves OW, IG
        Valve ZK has flow rate=0; tunnels lead to valves EW, WC
        Valve IG has flow rate=16; tunnels lead to valves OJ, VQ
        Valve WC has flow rate=22; tunnels lead to valves VD, ZK
        Valve EW has flow rate=18; tunnels lead to valves OJ, ZK
        Valve FP has flow rate=8; tunnel leads to valve GB
        Valve JF has flow rate=23; tunnel leads to valve VD
        Valve BL has flow rate=0; tunnels lead to valves AA, ZD
        Valve BZ has flow rate=0; tunnels lead to valves QK, JA
        Valve KH has flow rate=0; tunnels lead to valves SJ, FC
        Valve FU has flow rate=0; tunnels lead to valves FC, MH
        Valve ZP has flow rate=0; tunnels lead to valves SA, FC
        Valve DZ has flow rate=0; tunnels lead to valves AA, MH
        Valve RI has flow rate=0; tunnels lead to valves LP, MH
        Valve AE has flow rate=0; tunnels lead to valves FC, AA
        Valve JA has flow rate=4; tunnels lead to valves MM, BZ, JR, ZI, QO
        Valve XP has flow rate=0; tunnels lead to valves ZD, ZI
        Valve GB has flow rate=0; tunnels lead to valves FP, SJ
        Valve AM has flow rate=0; tunnels lead to valves ZD, RL
        Valve MH has flow rate=3; tunnels lead to valves VJ, DZ, JR, FU, RI
        Valve QK has flow rate=0; tunnels lead to valves BZ, SA
        Valve AA has flow rate=0; tunnels lead to valves DZ, CZ, BL, AE, BN
        Valve MJ has flow rate=0; tunnels lead to valves VN, VH
        Valve QO has flow rate=0; tunnels lead to valves CZ, JA
        Valve MM has flow rate=0; tunnels lead to valves FC, JA
        Valve VN has flow rate=17; tunnels lead to valves FV, MJ
        Valve OW has flow rate=0; tunnels lead to valves SJ, VQ
        Valve ZI has flow rate=0; tunnels lead to valves XP, JA
        Valve VJ has flow rate=0; tunnels lead to valves KJ, MH
        Valve KQ has flow rate=0; tunnels lead to valves XZ, KJ
        Valve FC has flow rate=12; tunnels lead to valves ZP, MM, KH, AE, FU
        Valve LP has flow rate=0; tunnels lead to valves SA, RI
        Valve VD has flow rate=0; tunnels lead to valves WC, JF
        Valve JR has flow rate=0; tunnels lead to valves MH, JA
        Valve VH has flow rate=0; tunnels lead to valves SA, MJ
        Valve CZ has flow rate=0; tunnels lead to valves AA, QO
        Valve SJ has flow rate=15; tunnels lead to valves KH, FV, GB, OW
        Valve FV has flow rate=0; tunnels lead to valves VN, SJ
        Valve XZ has flow rate=0; tunnels lead to valves LR, KQ
        Valve KJ has flow rate=9; tunnels lead to valves KQ, VJ
        Valve ZD has flow rate=13; tunnels lead to valves XP, BL, AM
    "};
}
