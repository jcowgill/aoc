use crate::vector::VectorExt;
use itertools::Itertools;
use nalgebra::Vector3;
use std::mem::replace;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Edge {
    dist_sq: i64,
    a: usize,
    b: usize,
}

#[derive(Clone, Debug)]
struct UnionFind {
    parents: Vec<usize>,
    unions_left: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: (0..n).collect(),
            unions_left: n - 1,
        }
    }

    /// Finds the set an item belongs to
    fn find(&mut self, mut item: usize) -> usize {
        assert!(item < self.parents.len());

        // Get ultimate parent
        let mut root = item;
        while self.parents[root] != root {
            root = self.parents[root];
        }

        // Update parents chain
        while self.parents[item] != root {
            item = replace(&mut self.parents[item], root);
        }

        root
    }

    /// Unions the sets containing the given items
    fn union(&mut self, a: usize, b: usize) {
        let a_parent = self.find(a);
        let b_parent = self.find(b);
        if a_parent != b_parent {
            self.parents[a_parent] = b_parent;
            self.unions_left -= 1;
        }
    }

    /// Has everything been combined into one set?
    fn is_done(&self) -> bool {
        self.unions_left == 0
    }
}

fn parse(input: &str) -> Vec<Vector3<i32>> {
    input
        .lines()
        .map(|line| Vector3::from_iterator(line.split(',').map(|n| n.parse().unwrap())))
        .collect()
}

fn edges(boxes: &[Vector3<i32>]) -> impl Iterator<Item = Edge> {
    (0..boxes.len()).array_combinations().map(|[a, b]| Edge {
        dist_sq: (boxes[a] - boxes[b]).cast().l2_squared_norm(),
        a,
        b,
    })
}

pub fn star1(input: &str) -> String {
    let boxes = parse(input);
    let max_edges = if boxes.len() <= 20 { 10 } else { 1000 };

    // Find the smallest 1000 edges
    let mut edges: Vec<_> = edges(&boxes).collect();
    edges.select_nth_unstable(max_edges);

    // Merge all the edges
    let mut sets = UnionFind::new(boxes.len());
    for edge in &edges[..max_edges] {
        sets.union(edge.a, edge.b);
    }

    let mut counts: Vec<_> = (0..boxes.len())
        .counts_by(|i| sets.find(i))
        .into_values()
        .collect();
    counts.select_nth_unstable_by(2, |a, b| b.cmp(a));
    (counts[0] * counts[1] * counts[2]).to_string()
}

pub fn star2(input: &str) -> String {
    let boxes = parse(input);

    // Merge all the edges
    let mut sets = UnionFind::new(boxes.len());
    for edge in edges(&boxes).sorted_unstable() {
        sets.union(edge.a, edge.b);
        if sets.is_done() {
            return (i64::from(boxes[edge.a].x) * i64::from(boxes[edge.b].x)).to_string();
        }
    }

    panic!("tree is not connected");
}
