use crate::direction::Direction;
use nalgebra::Vector2;
use std::collections::HashMap;

/// State of an individual node
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

/// State of the virus system
struct SporificaState {
    /// Set of nodes currently infected
    infection_map: HashMap<Vector2<i32>, NodeState>,

    /// Current virus node
    virus_node: Vector2<i32>,

    /// Current virus direction
    virus_dir: Direction,
}

/// Parses the input map into the initial infection state
///  The middle of the map is NOT given 0,0 for ease of parsing
fn parse_state<'a, I: Iterator<Item = &'a str>>(lines: I) -> SporificaState {
    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for line in lines {
        // Validate line and initialize width
        assert_eq!(line.chars().filter(|&c| c != '.' && c != '#').count(), 0);
        if width == 0 {
            width = line.len();
        } else {
            assert_eq!(width, line.len());
        };

        // Insert pre-infected nodes into the map
        map.extend(line.chars().enumerate().filter_map(|(i, c)| {
            if c == '#' {
                Some((Vector2::new(i as i32, height), NodeState::Infected))
            } else {
                None
            }
        }));
        height += 1;
    }

    SporificaState {
        infection_map: map,
        virus_node: Vector2::new(width as i32 / 2, height / 2),
        virus_dir: Direction::North,
    }
}

/// Simulates on burst of virus activity
///  state_transform is the function which determines the next node state
///  Returns true if a node was newly infected
fn burst<F>(s: &mut SporificaState, state_transform: &F) -> bool
where
    F: Fn(NodeState) -> NodeState,
{
    // Get node state
    let node_state = *s
        .infection_map
        .get(&s.virus_node)
        .unwrap_or(&NodeState::Clean);

    // Change virus direction (always the same rules)
    match node_state {
        NodeState::Clean => s.virus_dir = s.virus_dir.anticlockwise(),
        NodeState::Weakened => (),
        NodeState::Infected => s.virus_dir = s.virus_dir.clockwise(),
        NodeState::Flagged => s.virus_dir = s.virus_dir.reverse(),
    };

    // Update state map
    let new_state = state_transform(node_state);
    if new_state == NodeState::Clean {
        s.infection_map.remove(&s.virus_node);
    } else {
        s.infection_map.insert(s.virus_node, new_state);
    };

    // Advance virus position
    s.virus_node += s.virus_dir.to_vec_neg();

    new_state == NodeState::Infected
}

/// Common star entry point
fn star_common<F>(input: &str, default_iterations: usize, state_transform: F) -> String
where
    F: Fn(NodeState) -> NodeState,
{
    // Extract iterations from first line (if given)
    let mut lines = input.lines().peekable();
    let iterations = match lines.peek().unwrap().parse::<usize>() {
        Ok(i) => {
            lines.next();
            i
        }
        Err(_) => default_iterations,
    };

    // Process iterations and count new infections
    let mut s = parse_state(lines);
    (0..iterations)
        .map(|_| burst(&mut s, &state_transform))
        .filter(|&i| i)
        .count()
        .to_string()
}

pub fn star1(input: &str) -> String {
    star_common(input, 10000, |ns| match ns {
        NodeState::Clean => NodeState::Infected,
        _ => NodeState::Clean,
    })
}

pub fn star2(input: &str) -> String {
    star_common(input, 10000000, |ns| match ns {
        NodeState::Clean => NodeState::Weakened,
        NodeState::Weakened => NodeState::Infected,
        NodeState::Infected => NodeState::Flagged,
        NodeState::Flagged => NodeState::Clean,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1A, "41");
    star_test!(example2a, star1, IN2, "5587");
    star_test!(me1, star1, ME, "5196");

    star_test!(example1b, star2, IN1B, "26");
    star_test!(example2b, star2, IN2, "2511944");
    star_test!(me2, star2, ME, "2511633");

    const IN1A: &str = indoc! {"
        70
        ..#
        #..
        ...
    "};

    const IN1B: &str = indoc! {"
        100
        ..#
        #..
        ...
    "};

    const IN2: &str = indoc! {"
        ..#
        #..
        ...
    "};

    const ME: &str = indoc! {"
        .#.....##..##..###.###..#
        ..##..######.#.###.##.#.#
        ###..#..#####.##.##.#...#
        ###......##..###.#...#.#.
        .#.###.##..#.####.#..#...
        ..#.#.#####...##.####.###
        ..#..#.#..###.#..###.###.
        #########...#....##..#.#.
        .###..#######..####...###
        #####...#..##...###..##..
        ..#......##.#....#...####
        .##.#..#####.#####.##.##.
        ####.##.###.#..#.#.#.....
        #....##.####.#.#..#.#.##.
        ###...##...#.###.#.#.####
        .#.#...#.#.##.##....##.#.
        #..##.#.#..#....###..####
        #####...#..#.###...##.###
        ##.#..####.###...#....###
        ###.#####.....#....#.##..
        ####.##.....######.#..#.#
        .#.....####.##...###..##.
        ....########.#..###.#..##
        ##.##..#...#...##.#....##
        .#.######.##....####.#.##
    "};
}
