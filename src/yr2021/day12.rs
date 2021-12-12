use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum NodeType {
    Start,
    End,
    Big,
    Small,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    ty: NodeType,
    edges: Vec<usize>,
}

fn parse_node_type(node_name: &str) -> NodeType {
    if node_name == "start" {
        NodeType::Start
    } else if node_name == "end" {
        NodeType::End
    } else if node_name.chars().all(|c| c.is_ascii_uppercase()) {
        NodeType::Big
    } else {
        NodeType::Small
    }
}

fn parse_graph(input: &str) -> Vec<Node> {
    let mut name_to_id = HashMap::new();
    let mut graph = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line
            .trim()
            .split('-')
            .map(|node_name| {
                *name_to_id.entry(node_name).or_insert_with(|| {
                    graph.push(Node {
                        ty: parse_node_type(node_name),
                        edges: Vec::new(),
                    });
                    graph.len() - 1
                })
            })
            .collect();
        assert_eq!(parts.len(), 2);

        for i in 0..2 {
            if !graph[parts[i]].edges.contains(&parts[1 - i]) {
                graph[parts[i]].edges.push(parts[1 - i]);
            }
        }
    }

    graph
}

fn graph_start(graph: &[Node]) -> usize {
    graph
        .iter()
        .position(|n| n.ty == NodeType::Start)
        .expect("no start node")
}

/// Finds the number of paths from the given node to the end node
fn search_node<T, F>(graph: &[Node], small_nodes: &T, recorder: F, id: usize) -> usize
where
    F: Fn(&T, usize) -> Option<T> + Copy,
{
    graph[id]
        .edges
        .iter()
        .map(|&edge| {
            match graph[edge].ty {
                // We don't revisit the start again
                NodeType::Start => 0,

                // We got to the end of this path
                NodeType::End => 1,

                // Always visit big nodes
                NodeType::Big => search_node(graph, small_nodes, recorder, edge),

                // Visit small nodes if the recorder allows it
                // TODO we could avoid the need to copy small_nodes
                // here if we poped values when the recursion returned
                NodeType::Small => {
                    if let Some(new_small_nodes) = recorder(small_nodes, edge) {
                        search_node(graph, &new_small_nodes, recorder, edge)
                    } else {
                        0
                    }
                }
            }
        })
        .sum()
}

pub fn star1(input: &str) -> String {
    let graph = parse_graph(input);
    let recorder = |small_nodes: &Vec<usize>, id| {
        if !small_nodes.contains(&id) {
            let mut new = Vec::with_capacity(small_nodes.len());
            new.extend_from_slice(small_nodes);
            new.push(id);
            Some(new)
        } else {
            None
        }
    };

    search_node(&graph, &Vec::new(), recorder, graph_start(&graph)).to_string()
}

pub fn star2(input: &str) -> String {
    let graph = parse_graph(input);
    let recorder = |(small_nodes, visited_twice): &(Vec<usize>, bool), id| {
        if !small_nodes.contains(&id) {
            let mut new = Vec::with_capacity(small_nodes.len());
            new.extend_from_slice(small_nodes);
            new.push(id);
            Some((new, *visited_twice))
        } else if !visited_twice {
            Some((small_nodes.clone(), true))
        } else {
            None
        }
    };

    search_node(&graph, &(Vec::new(), false), recorder, graph_start(&graph)).to_string()
}
