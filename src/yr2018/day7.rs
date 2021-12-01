use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use lazy_static::lazy_static;
use regex::Regex;

/// Parses the input list of dependencies
fn parse_input<'a, I: Iterator<Item = &'a str>>(lines: I) -> HashMap<char, Vec<char>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$").unwrap();
    }

    let mut result = HashMap::new();

    for line in lines {
        let caps = RE.captures(line).unwrap();
        let from = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let to = caps.get(2).unwrap().as_str().chars().next().unwrap();

        result.entry(from).or_insert_with(Vec::new).push(to);
    }

    result
}

/// Internal state of Kahn's topological sorting algorithm
#[derive(Debug)]
struct TopologicalSort<'a> {
    /// Immutable set of edges describing the graph being processed
    edges: &'a HashMap<char, Vec<char>>,

    /// Counts the number of incoming edges for each node
    incoming_count: HashMap<char, usize>,

    /// Current set of open nodes (can be processed immediately)
    open: BinaryHeap<Reverse<char>>,
}

impl<'a> TopologicalSort<'a> {
    /// Constructs a new state
    pub fn new(edges: &HashMap<char, Vec<char>>) -> TopologicalSort {
        // Index number of incoming edges for each node
        let mut incoming_count = HashMap::new();
        for (node, outgoing) in edges {
            incoming_count.entry(*node).or_default();
            for dest in outgoing {
                *incoming_count.entry(*dest).or_default() += 1;
            }
        }

        // Open set initialised to nodes with no incoming edges
        let open = incoming_count
            .iter()
            .filter(|(_, &c)| c == 0)
            .map(|(&n, _)| Reverse(n))
            .collect();

        TopologicalSort {
            edges,
            incoming_count,
            open,
        }
    }

    /// Returns true if the topological sort is complete
    ///  If this returns false and there is nothing to do, the graph contains a cycle
    pub fn is_complete(&self) -> bool {
        self.open.is_empty() && self.incoming_count.values().all(|&c| c == 0)
    }

    /// Returns the next open node and removes it from the open nodes set
    pub fn take_open_node(&mut self) -> Option<char> {
        self.open.pop().map(|Reverse(c)| c)
    }

    /// Closes a node
    ///  At this point, new nodes are added to the open set if possible
    pub fn close_node(&mut self, node: char) {
        // Add any nodes which become open, and update remaining
        // incoming edge counts
        for dest in self
            .edges
            .get(&node)
            .map(|v| v.iter())
            .unwrap_or_else(|| [].iter())
        {
            let index = self.incoming_count.get_mut(dest).unwrap();

            assert!(*index > 0);
            *index -= 1;

            if *index == 0 {
                self.open.push(Reverse(*dest));
            }
        }
    }
}

/// Generate topological sort of input data
pub fn star1(input: &str) -> String {
    let edges = parse_input(input.lines());
    let mut top_sort_state = TopologicalSort::new(&edges);
    let mut result = String::new();

    // For a simple topological sort, immediately close the next open node
    while let Some(c) = top_sort_state.take_open_node() {
        result.push(c);
        top_sort_state.close_node(c);
    }

    assert!(top_sort_state.is_complete());
    result
}

/// Return time it takes for elves to process steps
pub fn star2(input: &str) -> String {
    // Parse input
    let (edges, workers, all_step_time) = if !input.starts_with("Step") {
        let int_parts: Vec<usize> = input.lines().take(2).map(|l| l.parse().unwrap()).collect();
        (
            parse_input(input.lines().skip(2)),
            int_parts[0],
            int_parts[1],
        )
    } else {
        (parse_input(input.lines()), 5, 60)
    };

    // Simulate parallel working
    let mut top_sort_state = TopologicalSort::new(&edges);
    let mut active_steps = BinaryHeap::with_capacity(workers);
    let mut now = 0;

    while !active_steps.is_empty() || !top_sort_state.is_complete() {
        // Start as many workers as possible
        while active_steps.len() < workers {
            if let Some(c) = top_sort_state.take_open_node() {
                active_steps.push(Reverse((
                    now + all_step_time + (c as usize - 'A' as usize + 1),
                    c,
                )));
            } else {
                break;
            }
        }

        // Advance time to the first step to finish
        now = (active_steps.peek().unwrap().0).0;

        // Close any nodes which have completed
        while let Some(&Reverse((close_time, c))) = active_steps.peek() {
            if now == close_time {
                top_sort_state.close_node(c);
                active_steps.pop();
            } else {
                break;
            }
        }
    }

    now.to_string()
}
