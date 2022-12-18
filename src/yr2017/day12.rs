use std::collections::HashMap;

/// Data structure which maps nodes to their node connections
type Nodes = HashMap<i32, Vec<i32>>;

/// Split the list of programs into a list of the nodes connected to each node
fn split_programs(input: &str) -> Nodes {
    input
        .lines()
        .map(|line| {
            let halves: Vec<&str> = line.splitn(2, "<->").collect();
            let node = halves[0].trim().parse().unwrap();
            let children = halves[1]
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect();
            (node, children)
        })
        .collect()
}

/// Partitions a set of nodes by separating all the nodes belonging to the same group as the
///  initial node given
/// Modifies given nodes list to remove elements in the group
/// Returns vector of elements in the group
fn partition_by_group(initial: i32, nodes: &mut Nodes) -> Vec<i32> {
    let mut open: Vec<i32> = Vec::new();
    let mut closed: Vec<i32> = Vec::new();

    // Populate open with initial node
    assert!(nodes.contains_key(&initial));
    open.push(initial);

    while !open.is_empty() {
        let current = open.pop().unwrap();

        // Ignore nodes we've already seen.
        if let Some(current_nodes) = nodes.remove(&current) {
            // Add current to closed list
            closed.push(current);

            // Add all node children to open list
            for child in current_nodes {
                open.push(child);
            }
        }
    }

    closed
}

/// Find number of nodes in first group
pub fn star1(input: &str) -> String {
    partition_by_group(0, &mut split_programs(input))
        .len()
        .to_string()
}

/// Find total number of groups
pub fn star2(input: &str) -> String {
    let mut nodes = split_programs(input);
    let mut groups = 0;

    while !nodes.is_empty() {
        // Partition by any node in the nodes list, ignoring the result
        partition_by_group(*nodes.keys().next().unwrap(), &mut nodes);
        groups += 1;
    }

    groups.to_string()
}
