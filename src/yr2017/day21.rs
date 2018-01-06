use std::collections::HashMap;

/*

/// Recursive tree structure representing an image
#[derive(Clone, Debug)]
enum ImageTree {
    /// Leaf node of a tree (either len = 4 or 9)
    Leaf(Vec<bool>),

    /// Recursive node (either len = 4 or 9)
    Node(Vec<ImageTree>),
}

/// A set of image transformation rules
type Ruleset = HashMap<Vec<bool>, ImageTree>;

impl ImageTree {
    /// Returns the number of "on" pixels in the image
    fn count_on(&self) -> usize {
        match *self {
            ImageTree::Leaf(ref v) => v.iter().filter(|&b| *b).count(),
            ImageTree::Node(ref v) => v.iter().map(ImageTree::count_on).sum()
        }
    }

    /// Transforms the current image according to the given ruleset
    fn expand(&mut self, rules: &Ruleset) {
        match *self {
            ImageTree::Leaf(_) => {
                let
                let new_tree = rules[v].clone();
                *self = new_tree
            },
            ImageTree::Node(ref mut v) => for i in v { i.expand(rules) }
        }
    }
}

/// Converts a simple image into an image tree
fn parse_image(image: &str) -> ImageTree {
    let bool_vec: Vec<Vec<bool>> = image
        .split('/')
        .map(|s| s.trim())
        .map(|p| p.chars().filter(|&c| c == '.' || c == '#').map(|c| c == '#').collect())
        .collect();

    // Validate line lengths
    for line in bool_vec { assert_eq!(line.len(), bool_vec.len()) }

    // Handle simple sizes
    match bool_vec.len() {
        2 => {
            bool_vec[0].append(&mut bool_vec[1]);
            ImageTree::Leaf(bool_vec[0])
        },
        3 => {
            bool_vec[0].append(&mut bool_vec[1]);
            bool_vec[0].append(&mut bool_vec[2]);
            ImageTree::Leaf(bool_vec[0])
        },
        4 => {
            ImageTree::Node(vec![
                ImageTree::Leaf(vec![bool_vec[0][0], bool_vec[0][1], bool_vec[1][0], bool_vec[1][1]]),
                ImageTree::Leaf(vec![bool_vec[0][2], bool_vec[0][3], bool_vec[1][2], bool_vec[1][3]]),
                ImageTree::Leaf(vec![bool_vec[2][0], bool_vec[2][1], bool_vec[3][0], bool_vec[3][1]]),
                ImageTree::Leaf(vec![bool_vec[2][2], bool_vec[2][3], bool_vec[3][2], bool_vec[3][3]])
            ])
        },
        _ => panic!("invalid tree parse size")
    }
}

/// Parses the list of rules from the input string
fn parse_rules(input: &str) -> Ruleset {
    input.lines().map(|line| {
        let parts: Vec<ImageTree> = line.split("=>").map(parse_image).collect();
        assert_eq!(parts.len(), 2);

        match parts[0] {
            ImageTree::Leaf(inner) => (inner, parts[1]),
            _ => panic!("ruleset keys must be size 2 or 3")
        }
    }).collect()
}

/// Rotates a leaf node 90 degrees clockwise
fn rotate_leaf(old: &[bool]) -> Vec<bool> {
    match old.len() {
        4 => vec![old[2], old[0],
                  old[3], old[1]],
        9 => vec![old[6], old[3], old[0],
                  old[7], old[4], old[1],
                  old[8], old[5], old[2]],
        _ => panic!("invalid leaf vector")
    }
}

/// Flips a leaf node
fn flip_leaf(old: &[bool]) -> Vec<bool> {
    match old.len() {
        4 => vec![old[2], old[3],
                  old[0], old[1]],
        9 => vec![old[6], old[7], old[8],
                  old[3], old[4], old[5],
                  old[0], old[1], old[2]],
        _ => panic!("invalid leaf vector")
    }
}

/// Expands the ruleset
fn expand_rules(old_rules: Ruleset) -> Ruleset {
    let new_rules = HashMap::new();
    for (key, value) in old_rules.drain() {
        let rot1 = rotate_leaf(&key);
        let rot2 = rotate_leaf(&rot1);
        let rot3 = rotate_leaf(&rot2);

        new_rules.insert(rot1, value.clone());
        new_rules.insert(rot2, value.clone());
        new_rules.insert(rot3, value.clone());
        new_rules.insert(flip_leaf(&key), value.clone());
        new_rules.insert(key, value);
    }

    new_rules
}

*/

/// An image represented as a square vector of monochrome pixels
#[derive(Clone, Debug)]
struct Image {
    data: Vec<bool>,
    size: usize
}

/// A simple linear transformation matrix
#[derive(Clone, Copy, Debug)]
struct LinearTransform(i8, i8, i8, i8);

// Some common transformations
const TRANSFORM_IDENTITY: LinearTransform   = LinearTransform(1, 0, 0, 1);
const TRANSFORM_ROTATE90: LinearTransform   = LinearTransform(0, 1, -1, 0);
const TRANSFORM_ROTATE180: LinearTransform  = LinearTransform(0, -1, -1, 0);
const TRANSFORM_ROTATE270: LinearTransform  = LinearTransform(0, 1, 1, 0);
const TRANSFORM_FLIP_VERT: LinearTransform  = LinearTransform(-1, 0, 0, 1);
const TRANSFORM_FLIP_HORIZ: LinearTransform = LinearTransform(1, 0, 0, -1);

/// List of image transformation rules
type Ruleset = HashMap<Image, Image>;

impl Image {
    /// Counts the number of "on" pixels
    fn count_on(&self) -> usize {
        self.0.iter().filter(|p| p).count()
    }

    /// Transforms an image according to the given linear transformation
    fn transform(&self, matrix: LinearTransform) -> Image {
        assert_eq!(self.size * self.size, self.data.len());

        let new_data = Vec::new();
        new_data.resize(self.data.len(), false);

        for y in 0..self.size {
            for x in 0..self.size {
                if self.data[y * self.size + x] {
                    let new_x = x * matrix.0 + y * matrix.1;
                    let new_y = x * matrix.2 + y * matrix.3;
                    new_data[new_y * self.size + new_x] = true;
                }
            }
        }

        Image { data: new_data, size: self.size }
    }

    /// Expands the image using the given ruleset
    fn expand_by_rules(&self, rules: &Ruleset) -> Image {
        // Rules are either
    }
}

impl FromStr for Image {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<bool> = image
            .chars()
            .filter(|&c| c == '.' || c == '#')
            .map(|c| c == '#')
            .collect();

        // Validate length
        let sqrt_len = (parsed.len() as f64).sqrt() as usize;
        if sqrt_len * sqrt_len != parsed.len() { return Err(()) }

        Ok(Image(parsed))
    }
}

/// Parses the list of rules from the input string
fn parse_rules(input: &str) -> Ruleset {
    input.lines().map(|line| {
        let parts: Vec<Image> = line.split("=>").map(str::parse).collect();
        assert_eq!(parts.len(), 2);
        (parts[0], parts[1])
    }).collect()
}

/// Expands the given ruleset to include extra transformations of keys
fn expand_rules(old_rules: Ruleset) -> Ruleset {
    let new_rules = HashMap::new();
    for (key, value) in old_rules.drain() {
        new_rules.insert(key, value.clone());
        new_rules.insert(key.transform(TRANSFORM_ROTATE90), value.clone());
        new_rules.insert(key.transform(TRANSFORM_ROTATE180), value.clone());
        new_rules.insert(key.transform(TRANSFORM_ROTATE270), value.clone());
        new_rules.insert(key.transform(TRANSFORM_FLIP_VERT), value.clone());
        new_rules.insert(key.transform(TRANSFORM_FLIP_HORIZ), value);
    }

    new_rules
}

/// Find pixel count after 5 iterations
pub fn star1(input: &str) -> String {
    let rules = expand_rules(parse_rules(input));
    let mut image: Image = ".#./..#/###".parse();

    // Run 5 growth iterations and count "on" pixels
    for _ in 0..5 { image = image.expand_by_rules(&rules) }
    image.count_on().to_string()
}
