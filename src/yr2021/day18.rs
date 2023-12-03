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
            Node::Leaf(v) => write!(f, "{v}"),
            Node::Pair(a, b) => write!(f, "[{a},{b}]"),
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
        Some(c) => panic!("syntax error {c}, expected {expected}"),
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
        Some(c) => panic!("unexpected char {c}"),
        None => panic!("unexpected end of string"),
    }
}

/// Parses a node from an input string
fn parse_node_str(input: &str) -> Node {
    let mut iter = input.chars().peekable();
    let result = parse_node(&mut iter);
    skip_whitespace(&mut iter);
    if let Some(c) = iter.next() {
        panic!("unexpected char {c}, expected end of string");
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "4140");
    star_test!(me1, star1, ME, "4480");

    star_test!(example1b, star2, IN1, "3993");
    star_test!(me2, star2, ME, "4676");

    const IN1: &str = indoc! {"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "};

    const ME: &str = indoc! {"
        [[[6,[0,5]],2],3]
        [[3,9],[[4,[5,5]],[9,4]]]
        [[3,[2,[0,7]]],[[1,[0,1]],[8,[2,4]]]]
        [[[[3,0],[8,1]],[[5,6],[3,5]]],[5,[[7,9],[5,1]]]]
        [[[9,5],1],[[2,6],[4,[4,6]]]]
        [2,[5,[3,0]]]
        [[[[3,8],9],[[9,2],[7,7]]],5]
        [[[[8,5],3],[0,[0,8]]],[9,[6,[8,1]]]]
        [[[[9,6],[9,1]],[[3,4],3]],[2,2]]
        [[3,[[7,7],2]],[[[9,9],[8,6]],2]]
        [[[[3,5],8],[4,[9,4]]],7]
        [[1,[6,[2,6]]],[[7,0],[[3,1],9]]]
        [[[[0,5],0],[0,[2,4]]],[7,[6,[1,6]]]]
        [0,[6,[3,1]]]
        [4,[[[1,2],0],[[6,3],6]]]
        [[[[4,3],[8,5]],[9,[0,5]]],[[[8,1],3],[8,8]]]
        [[3,[6,7]],[[[1,4],[2,1]],[9,[2,8]]]]
        [[[[3,0],[4,5]],4],[[3,[5,7]],6]]
        [[8,[5,[3,9]]],[[[1,9],[8,7]],7]]
        [[[0,[3,4]],[[0,1],6]],[1,5]]
        [6,[[9,4],[9,0]]]
        [[4,9],[[[0,9],9],8]]
        [5,[0,[3,6]]]
        [[[[3,9],[1,1]],[4,0]],[4,[[5,7],6]]]
        [[[6,[2,1]],[4,[3,3]]],[[6,[5,6]],[7,[5,0]]]]
        [[[8,4],[[1,2],[2,0]]],[[6,[5,6]],[6,4]]]
        [[[5,9],7],[[[3,4],[2,1]],2]]
        [[[8,8],5],[6,[[9,1],1]]]
        [[[[3,4],1],[1,[4,2]]],[[[9,1],[2,0]],3]]
        [[[5,[1,7]],[3,[9,7]]],[[2,0],[[7,7],5]]]
        [[[6,9],6],[6,[[4,2],[5,3]]]]
        [[[[6,5],[5,0]],[[6,9],7]],6]
        [[[[2,8],[3,2]],[8,8]],[[[4,6],7],[2,[2,3]]]]
        [[[3,4],1],5]
        [[6,[[7,4],8]],[[6,[9,2]],9]]
        [[[[0,4],0],[[3,1],[3,6]]],[0,[[2,8],[5,5]]]]
        [[5,[[1,2],6]],[[[1,9],[2,2]],[9,[1,0]]]]
        [[[[7,7],5],[[0,0],[0,5]]],[[5,4],[5,[8,1]]]]
        [[[[8,7],6],[0,[0,3]]],[2,[[9,8],0]]]
        [6,7]
        [[[[5,2],[2,2]],[[8,1],[2,7]]],2]
        [[[1,0],[[9,8],[2,1]]],4]
        [[[[5,2],[2,0]],7],[[2,[4,4]],[3,3]]]
        [[8,3],[[7,2],[[1,5],9]]]
        [[[[9,4],[7,5]],[[4,9],1]],[[[0,6],[1,6]],[[4,2],3]]]
        [4,[5,[5,3]]]
        [[1,[[0,1],6]],[[[8,8],6],0]]
        [[[0,3],[1,2]],[[6,8],[7,[4,7]]]]
        [3,[[[5,8],[6,9]],[8,[5,4]]]]
        [[1,1],[[8,[7,0]],9]]
        [2,2]
        [[3,[[0,4],1]],[[7,[2,3]],[8,4]]]
        [[[0,2],[5,[5,6]]],[[[8,1],[5,2]],8]]
        [[[[9,5],6],[[7,2],[2,1]]],[[[0,0],0],[[7,9],[9,1]]]]
        [[1,7],[1,[[6,3],8]]]
        [4,[[[7,4],1],[[1,9],6]]]
        [[[[7,8],9],3],[[[5,0],9],8]]
        [[[1,3],[[8,3],[1,9]]],[[[6,2],5],5]]
        [[8,[[6,6],[1,4]]],[[2,[9,0]],[9,[2,2]]]]
        [[0,[[1,6],8]],[[[3,7],[9,2]],[3,[3,6]]]]
        [[0,[5,6]],[[[6,5],1],8]]
        [[[[9,0],[1,1]],[[1,0],1]],[[[2,5],7],[5,[0,3]]]]
        [2,[2,[[3,8],[5,5]]]]
        [[[[4,0],2],[4,2]],[[5,[8,1]],[[2,7],6]]]
        [[[1,[3,5]],6],0]
        [[[3,3],[1,[2,1]]],[[3,[1,1]],[7,0]]]
        [[[[8,5],3],[2,[3,0]]],[[[4,8],[8,8]],[9,[2,0]]]]
        [[[[8,3],[9,8]],[[8,3],2]],[5,[5,8]]]
        [[[1,9],8],[[[0,3],[9,1]],[[9,1],[4,2]]]]
        [[[4,1],6],[[5,2],[4,2]]]
        [[7,[5,[3,0]]],[[0,[0,1]],[1,[9,6]]]]
        [[[[0,3],[8,9]],1],[6,2]]
        [[5,[7,8]],[[5,3],[[9,8],[6,2]]]]
        [[[[3,6],8],9],[[3,[6,2]],[0,1]]]
        [[[5,3],[6,[0,3]]],6]
        [[[2,9],7],[[[4,2],9],8]]
        [[[7,[3,5]],[6,[5,9]]],[[[3,2],[5,6]],1]]
        [[0,[[9,1],6]],8]
        [[[[3,5],[2,2]],[[9,8],9]],[8,8]]
        [[[[6,9],[2,0]],[6,1]],[1,5]]
        [[[0,[9,9]],[[3,1],9]],[[[4,8],2],[3,[4,8]]]]
        [[[5,3],[6,3]],[6,[[2,7],[3,2]]]]
        [[[5,[7,3]],8],[[2,[0,0]],[9,5]]]
        [[[7,[4,3]],[[9,6],[5,7]]],[[[1,8],[3,0]],[[2,4],[7,3]]]]
        [[[3,[2,1]],[1,2]],[7,[[5,4],8]]]
        [[4,[[2,6],[7,9]]],[[[1,8],3],[[0,1],[5,4]]]]
        [[[4,0],4],[3,7]]
        [[[4,[7,7]],5],[2,3]]
        [[[[7,7],[9,3]],8],[[[3,8],6],[[1,2],[9,6]]]]
        [[7,[[7,5],[7,8]]],[2,[[4,3],[6,3]]]]
        [[[7,3],[[8,2],[6,5]]],1]
        [[8,[7,[9,6]]],[5,[[2,2],2]]]
        [[[3,[8,9]],[[3,8],[3,3]]],[4,[7,[5,0]]]]
        [[[[2,0],[1,0]],3],[6,[[1,3],[5,4]]]]
        [[1,8],[8,[[6,7],3]]]
        [4,[[[3,4],[7,8]],[7,[7,1]]]]
        [[[4,8],[9,[7,4]]],6]
        [[[7,3],[[2,9],7]],[[[2,5],8],[2,5]]]
        [7,[[[8,7],0],[[3,1],6]]]
        [[6,[2,[3,2]]],[[[5,4],2],[[2,6],[8,4]]]]
    "};
}
