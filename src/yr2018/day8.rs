/// A tree node
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>
}

impl Node {
    /// Creates a node from an iterator by partially consuming an iterator
    fn from_iter_part<I: Iterator<Item=u32>>(iter: &mut I) -> Option<Node> {
        let child_count = iter.next()?;
        let meta_count = iter.next()?;

        let children = (0..child_count).map(|_| Node::from_iter_part(iter)).collect::<Option<_>>()?;
        let metadata = (0..meta_count).map(|_| iter.next()).collect::<Option<_>>()?;

        Some(Node { children, metadata })
    }

    /// Returns the part1 sum of metadata of this and all child nodes
    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().sum::<u32>() +
            self.children.iter().map(|c| c.metadata_sum()).sum::<u32>()
    }

    /// Returns the part2 value of this node
    fn value(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            let child_values: Vec<u32> = self.children.iter().map(|c| c.value()).collect();
            self.metadata.iter().filter_map(|v| child_values.get((v - 1) as usize)).sum()
        }
    }
}

/// Parses input into the root node
fn parse_root_node(input: &str) -> Node {
    let mut iter = input.split_whitespace().map(|i| i.parse().unwrap());
    Node::from_iter_part(&mut iter).filter(|_| iter.next().is_none()).unwrap()
}

pub fn star1(input: &str) -> String {
    parse_root_node(input).metadata_sum().to_string()
}

pub fn star2(input: &str) -> String {
    parse_root_node(input).value().to_string()
}
