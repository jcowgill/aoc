use itertools::Itertools;
use std::collections::{BinaryHeap, HashSet};

/// Describes a type of bug (A, B, C or D)
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct BugType(u8);

/// Describes a position in the hallway
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Hallway(u8);

/// Describes a bug starting position
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct StartPos(u8);

type StartState = Vec<(BugType, StartPos)>;
type HallwayState = Vec<(BugType, Hallway)>;

impl BugType {
    /// Enumerate all types of bug
    fn all_types() -> impl Iterator<Item = BugType> {
        (0..4).map(BugType)
    }

    /// Calculates the cost per move of this bug type
    fn cost(self) -> i32 {
        10_i32.pow(self.0.into())
    }

    /// Returns the hallway index this bug is destined for
    fn goal(self) -> Hallway {
        Hallway((self.0 + 1) * 2)
    }
}

impl Hallway {
    /// Enumerate all valid hallway destinations
    fn all_destinations() -> impl Iterator<Item = Hallway> {
        [0, 1, 3, 5, 7, 9, 10].into_iter().map(Hallway)
    }

    /// Would this hallway block a move from src to dst?
    fn would_block(self, src: Hallway, dst: Hallway) -> bool {
        if src < dst {
            self > src && self <= dst
        } else {
            self >= dst && self < src
        }
    }

    /// Computes the absolute distance between two hallways
    fn abs_diff(self, other: Hallway) -> u8 {
        if self < other {
            other.0 - self.0
        } else {
            self.0 - other.0
        }
    }

    /// Computes the penalty for a bug to move to this hallway one way
    fn penalty(self, src: Hallway, bt: BugType) -> i32 {
        let left = src.0.min(bt.goal().0);
        let right = src.0.max(bt.goal().0);

        if self.0 < left {
            (left - self.0) as i32 * bt.cost()
        } else if self.0 > right {
            (self.0 - right) as i32 * bt.cost()
        } else {
            0
        }
    }
}

impl StartPos {
    /// Constructs a starting position from a room and row number
    fn from_room_row(room: u8, row: u8) -> StartPos {
        StartPos(room + row * 4)
    }

    /// Returns the room id of this start position
    fn room(self) -> u8 {
        self.0 % 4
    }

    /// Returns the row of this start position
    fn row(self) -> u8 {
        self.0 / 4
    }

    /// Returns the hallway this bug would enter when leaving the room
    fn entering_hallway(self) -> Hallway {
        BugType(self.room()).goal()
    }
}

/// Remove bugs from the start state which are already in the correct position
fn remove_complete_bugs(state: &mut StartState, depth: u8) {
    for bug_type in BugType::all_types() {
        for row in (0..depth).rev() {
            if let Some(i) = state
                .iter()
                .position(|&b| b == (bug_type, StartPos::from_room_row(bug_type.0, row)))
            {
                state.swap_remove(i);
            } else {
                break;
            }
        }
    }
}

fn star_common(mut initial_state: StartState) -> String {
    assert_eq!(initial_state.len() % 4, 0);
    let depth = (initial_state.len() / 4) as u8;
    remove_complete_bugs(&mut initial_state, depth);
    initial_state.sort_unstable();

    // Calculate "perfect" cost (all bugs go directly to their
    // destinations). This is made up of the cost to move each bug
    // from it's start position into its room plus the cost of moving
    // down to the final position within a room.
    let cost1: i32 = initial_state
        .iter()
        .map(|(bt, pos)| {
            (pos.row() + 1 + pos.entering_hallway().abs_diff(bt.goal())) as i32 * bt.cost()
        })
        .sum();
    let cost2: i32 = initial_state
        .iter()
        .counts_by(|(bt, _)| bt)
        .into_iter()
        .map(|(bt, n)| (n * (n + 1) / 2) as i32 * bt.cost())
        .sum();
    let perfect_cost = cost1 + cost2;

    // All states in either list are guaranteed to be relaxed
    let mut open = BinaryHeap::new();
    let mut closed = HashSet::new();
    open.push((0, initial_state, HallwayState::new()));

    // Find path with the lowest penalty from the perfect cost
    while let Some((penalty, start_state, hallway_state)) = open.pop() {
        if closed.insert((start_state.clone(), hallway_state.clone())) {
            if start_state.is_empty() && hallway_state.is_empty() {
                return (perfect_cost - penalty).to_string();
            }

            for (i, &(bt, p)) in start_state.iter().enumerate() {
                // Is this bug blocked from leaving?
                if start_state
                    .iter()
                    .any(|(_, p2)| p.room() == p2.room() && p.row() > p2.row())
                {
                    continue;
                }

                // Enumerate all hallway moves this bug could make
                for dst in Hallway::all_destinations() {
                    // Are we blocked from getting here?
                    if hallway_state
                        .iter()
                        .any(|(_, h)| h.would_block(p.entering_hallway(), dst))
                    {
                        continue;
                    }

                    // Compute new state
                    let new_penalty = penalty - 2 * dst.penalty(p.entering_hallway(), bt);
                    let mut new_start_state = start_state.clone();
                    new_start_state.remove(i);
                    let mut new_hallway_state = hallway_state.clone();
                    new_hallway_state.push((bt, dst));

                    // Remove bugs in hallways which can move to their
                    // destinations immediately
                    while let Some(i) = new_hallway_state.iter().position(|&(bt, h)| {
                        let hallway_block = new_hallway_state
                            .iter()
                            .any(|(_, h2)| h2.would_block(h, bt.goal()));
                        let enter_block = new_start_state
                            .iter()
                            .any(|(_, s)| s.entering_hallway() == bt.goal());

                        !hallway_block && !enter_block
                    }) {
                        new_hallway_state.swap_remove(i);
                    }

                    new_hallway_state.sort_unstable();

                    // Relax before pushing
                    open.push((new_penalty, new_start_state, new_hallway_state));
                }
            }
        }
    }

    panic!("no way to reach final state")
}

fn parse_start_pos((i, c): (usize, char)) -> (BugType, StartPos) {
    (BugType(c as u8 - b'A'), StartPos(i as u8))
}

pub fn star1(input: &str) -> String {
    star_common(
        input
            .chars()
            .filter(|c| ('A'..='D').contains(c))
            .enumerate()
            .map(parse_start_pos)
            .collect(),
    )
}

pub fn star2(input: &str) -> String {
    let in_chars = input.chars().filter(|c| ('A'..='D').contains(c));
    let spliced_chars = in_chars
        .clone()
        .take(4)
        .chain("DCBADBAC".chars())
        .chain(in_chars.clone().skip(4));
    star_common(spliced_chars.enumerate().map(parse_start_pos).collect())
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "12521");
    star_test!(me1, star1, ME, "16244");

    star_test!(example1b, star2, IN1, "44169");
    star_test!(me2, star2, ME, "43226");

    const IN1: &str = indoc! {"
        #############
        #...........#
        ###B#C#B#D###
          #A#D#C#A#
          #########
    "};

    const ME: &str = indoc! {"
        #############
        #...........#
        ###D#D#B#A###
          #B#C#A#C#
          #########
    "};
}
