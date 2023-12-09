use std::collections::HashMap;

use num::Integer;

type Node = u16;

const AAA: Node = 0;
const ZZZ: Node = 26 * 26 * 26 - 1;

fn parse_node(node: &str) -> Node {
    node.bytes()
        .map(|b| {
            if b.is_ascii_digit() {
                b - b'0' + b'A'
            } else {
                b
            }
        })
        .filter(u8::is_ascii_uppercase)
        .fold(0, |acc, b| acc * 26 + u16::from(b - b'A'))
}

fn parse_input(input: &str) -> (Vec<bool>, HashMap<Node, (Node, Node)>) {
    let (dirs, nodes) = input.split_once("\n\n").unwrap();
    (
        dirs.chars().map(|c| c == 'R').collect(),
        nodes
            .lines()
            .map(|line| {
                let parts: Vec<_> = line.split(&['=', ',']).map(parse_node).collect();
                (parts[0], (parts[1], parts[2]))
            })
            .collect(),
    )
}

fn search(dirs: &[bool], nodes: &HashMap<Node, (Node, Node)>, start: Node, end: Node) -> usize {
    dirs.iter()
        .cycle()
        .scan(start, |prev, &right| {
            let (l, r) = nodes[prev];
            *prev = if right { r } else { l };
            Some(*prev)
        })
        .take(50_000)
        .position(|n| n == end)
        .unwrap()
        + 1
}

pub fn star1(input: &str) -> String {
    let (dirs, nodes) = parse_input(input);
    search(&dirs, &nodes, AAA, ZZZ).to_string()
}

pub fn star2(input: &str) -> String {
    let (dirs, nodes) = parse_input(input);
    nodes
        .keys()
        .filter(|&k| k % 26 == 25)
        .map(|&k| search(&dirs, &nodes, k, k))
        .fold(1, |acc, c| acc.lcm(&c))
        .to_string()
}
