use itertools::Itertools;
use std::fmt;
use std::iter::Peekable;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Node {
    Leaf(u8),
    Pair(Box<Node>, Box<Node>),
}

impl Node {
    /// Adds two nodes together without reduction
    fn add(self, other: Node) -> Node {
        Node::Pair(Box::new(self), Box::new(other))
    }

    /// Reduces this node as much as possible
    fn reduce(&mut self) {
        loop {
            self.explode(0);
            if !self.split() {
                break;
            }
        }
    }

    /// Adds two nodes together and then reduces the node
    fn add_reduce(self, other: Node) -> Node {
        let mut result = self.add(other);
        result.reduce();
        result
    }

    /// Tries to explode the leftmost pair under this node
    ///
    /// depth is the current depth of this node
    ///
    /// Returns the values to the left and right which need adding to
    /// surrounding nodes.
    fn explode(&mut self, depth: usize) -> (Option<u8>, Option<u8>) {
        match self {
            Node::Leaf(_) => (None, None),
            Node::Pair(a, b) => {
                if depth >= 4 {
                    // Explode this pair
                    let result = (Some(a.unwrap_leaf()), Some(b.unwrap_leaf()));
                    *self = Node::Leaf(0);
                    result
                } else {
                    // Check if children need exploding and handle fixups
                    let (left_residual, left_fixup) = a.explode(depth + 1);
                    if let Some(r) = left_fixup {
                        *b.left_leaf() += r;
                    }

                    let (right_fixup, right_residual) = b.explode(depth + 1);
                    if let Some(l) = right_fixup {
                        *a.right_leaf() += l;
                    }

                    (left_residual, right_residual)
                }
            }
        }
    }

    /// Tries to split a single leaf under this node
    ///
    /// Returns true if a leaf was split
    fn split(&mut self) -> bool {
        match self {
            &mut Node::Leaf(v) if v >= 10 => {
                *self = Node::Leaf(v / 2).add(Node::Leaf((v + 1) / 2));
                true
            }
            Node::Leaf(_) => false,
            Node::Pair(a, b) => a.split() || b.split(),
        }
    }

    fn unwrap_leaf(&self) -> u8 {
        match self {
            Node::Leaf(v) => *v,
            Node::Pair(_, _) => panic!("node isn't a leaf"),
        }
    }

    /// Returns a reference to the leftmost leaf under this node
    fn left_leaf(&mut self) -> &mut u8 {
        match self {
            Node::Leaf(v) => v,
            Node::Pair(l, _) => l.left_leaf(),
        }
    }

    /// Returns a reference to the leftmost leaf under this node
    fn right_leaf(&mut self) -> &mut u8 {
        match self {
            Node::Leaf(v) => v,
            Node::Pair(_, r) => r.right_leaf(),
        }
    }

    /// Returns the magnitude of this node
    fn magnitude(&self) -> u32 {
        match self {
            Node::Leaf(v) => (*v).into(),
            Node::Pair(l, r) => l.magnitude() * 3 + r.magnitude() * 2,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Node::Leaf(v) => write!(f, "{}", v),
            Node::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

/// Skip whitespace in a char iterator
fn skip_whitespace(input: &mut Peekable<impl Iterator<Item = char>>) {
    while input.next_if(char::is_ascii_whitespace).is_some() {}
}

/// Consumes the given token
fn consume_token(input: &mut Peekable<impl Iterator<Item = char>>, expected: char) {
    skip_whitespace(input);
    match input.next() {
        Some(c) if c == expected => (),
        Some(c) => panic!("syntax error {}, expected {}", c, expected),
        None => panic!("unexpected end of string"),
    }
}

/// Parses a node from a peekable char iterator
fn parse_node(input: &mut Peekable<impl Iterator<Item = char>>) -> Node {
    skip_whitespace(input);
    match input.peek() {
        Some('[') => {
            consume_token(input, '[');
            let left = parse_node(input);
            consume_token(input, ',');
            let result = left.add(parse_node(input));
            consume_token(input, ']');
            result
        }
        Some(c) if c.is_ascii_digit() => {
            Node::Leaf(input.next().unwrap().to_digit(10).unwrap() as u8)
        }
        Some(c) => panic!("unexpected char {}", c),
        None => panic!("unexpected end of string"),
    }
}

/// Parses a node from an input string
fn parse_node_str(input: &str) -> Node {
    let mut iter = input.chars().peekable();
    let result = parse_node(&mut iter);
    skip_whitespace(&mut iter);
    if let Some(c) = iter.next() {
        panic!("unexpected char {}, expected end of string", c);
    }
    result
}

pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(parse_node_str)
        .reduce(Node::add_reduce)
        .map_or(0, |n| n.magnitude())
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .map(parse_node_str)
        .tuple_combinations()
        .map(|(l, r)| {
            l.clone()
                .add_reduce(r.clone())
                .magnitude()
                .max(r.add_reduce(l).magnitude())
        })
        .max()
        .unwrap_or(0)
        .to_string()
}
