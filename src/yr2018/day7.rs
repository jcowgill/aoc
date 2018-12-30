use std::collections::{BTreeSet, HashMap};

use lazy_static::lazy_static;
use regex::Regex;

/// Parses the input list of dependencies
fn parse_input(input: &str) -> HashMap<char, Vec<char>> {
    lazy_static! {
        static ref re: Regex = Regex::new("^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();
    }

    let mut result = HashMap::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let from = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let to   = caps.get(2).unwrap().as_str().chars().next().unwrap();

        result.entry(from).or_insert(Vec::new()).push(to);
    }

    result
}

/// Create an index counting the number of incoming edges in each node
///  In addition, the return map will contain every node (even nodes
///  with 0 incoming edges)
fn index_incoming_edges(edges: &HashMap<char, Vec<char>>) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    for (node, outgoing) in edges {
        result.entry(*node).or_default();
        for dest in outgoing {
            *result.entry(*dest).or_default() += 1;
        }
    }

    result
}

/// Generate an ordering from a set of edges using Kahn's algorithm
fn generate_ordering(edges: &HashMap<char, Vec<char>>) -> Option<Vec<char>> {
    let mut incoming_edges = index_incoming_edges(edges);
    let mut output = Vec::new();

    // Open set initially contains all nodes with no incoming edges
    let mut open: BTreeSet<char> = incoming_edges.iter().filter(|(_, &c)| c == 0).map(|(&n, _)| n).collect();

    while !open.is_empty() {
        // Pop the first (lexicographical smallest) node in the open set
        let current = *open.iter().next().unwrap();
        open.remove(&current);
        output.push(current);

        // Remove all outgoing edges of this node and add any new
        // nodes to the open set
        for dest in edges.get(&current).map(|v| v.iter()).unwrap_or([].iter()) {
            let index = incoming_edges.get_mut(dest).unwrap();

            assert!(*index > 0);
            *index -= 1;

            if *index == 0 {
                open.insert(*dest);
            }
        }
    }

    // The output should now contain every node (unless there's a cycle)
    if incoming_edges.len() == output.len() {
        Some(output)
    } else {
        None
    }
}

pub fn star1(input: &str) -> String {
    generate_ordering(&parse_input(input)).unwrap().into_iter().collect()
}

pub fn star2(_input: &str) -> String {
    unimplemented!()
}
