use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Node {
    File(u64),
    Dir(HashMap<String, Node>),
}

fn parse_input(input: &str) -> Node {
    let mut lines = input.lines().peekable();
    let mut root_map = HashMap::new();
    let mut dirs = Vec::new();

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd ") {
            match line.split_at(5).1 {
                "/" => dirs.clear(),
                ".." => {
                    dirs.pop();
                }
                dir => dirs.push(dir),
            }
        } else {
            assert_eq!(line, "$ ls");

            // Find the right map
            let mut map = &mut root_map;
            for &dir in dirs.iter() {
                if let Some(Node::Dir(ref mut dir)) = map.get_mut(dir) {
                    map = dir;
                } else {
                    panic!("{} is not a directory", dir);
                }
            }

            // Populate it
            while let Some(ls_line) = lines.next_if(|l| !l.starts_with('$')) {
                let (size, name) = ls_line.split_once(' ').unwrap();
                map.insert(
                    name.to_string(),
                    if size == "dir" {
                        Node::Dir(HashMap::new())
                    } else {
                        Node::File(size.parse().unwrap())
                    },
                );
            }
        }
    }

    Node::Dir(root_map)
}

fn dir_sizes(node: &Node, sizes: &mut Vec<u64>) -> u64 {
    match node {
        Node::File(size) => *size,
        Node::Dir(map) => {
            let my_size = map.values().map(|n| dir_sizes(n, sizes)).sum();
            sizes.push(my_size);
            my_size
        }
    }
}

fn parse_sizes(input: &str) -> (Vec<u64>, u64) {
    let mut sizes = Vec::new();
    let root_size = dir_sizes(&parse_input(input), &mut sizes);
    (sizes, root_size)
}

pub fn star1(input: &str) -> String {
    parse_sizes(input)
        .0
        .into_iter()
        .filter(|&s| s <= 100000)
        .sum::<u64>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let (sizes, root_size) = parse_sizes(input);
    let space_needed = root_size - 40000000;
    sizes
        .into_iter()
        .filter(|&s| s >= space_needed)
        .min()
        .unwrap()
        .to_string()
}
