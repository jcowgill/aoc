use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::iter;
use std::str::FromStr;

/// Direction the tape cursor can move
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum TapeMove {
    Left,
    Right,
}

impl FromStr for TapeMove {
    type Err = ();
    fn from_str(s: &str) -> Result<TapeMove, ()> {
        if s == "left" {
            Ok(TapeMove::Left)
        } else if s == "right" {
            Ok(TapeMove::Right)
        } else {
            Err(())
        }
    }
}

/// Type used for the transition mapping dictionary
type TransitionMap<State, Symbol> = HashMap<(State, Symbol), (State, Symbol, TapeMove)>;

/// A turing machine with infinte tape in both directions
struct TuringMachine<State, Symbol> {
    /// Blank symbol (constant)
    blank: Symbol,

    /// Map of state transitions (constant)
    transitions: TransitionMap<State, Symbol>,

    /// Current state
    state: State,

    /// Current cursor position
    ///  The position must be within the tape size or one off the end
    cursor: usize,

    /// Tape contents
    tape: VecDeque<Symbol>,
}

impl<State: Copy + Eq + Hash, Symbol: Copy + Eq + Hash> TuringMachine<State, Symbol> {
    /// Creates a new turing machine
    fn new(
        blank: Symbol,
        transitions: TransitionMap<State, Symbol>,
        initial_state: State,
        mut initial_tape: Vec<Symbol>,
    ) -> Self {
        assert!(!initial_tape.contains(&blank));

        // Cursor must point to a valid tape position
        if initial_tape.is_empty() {
            initial_tape.push(blank);
        };

        TuringMachine {
            blank,
            transitions,
            state: initial_state,
            cursor: 0,
            tape: initial_tape.into(),
        }
    }

    /// Executes one step of the turing machine
    ///  Returns true if the machine is still running
    fn step(&mut self) -> bool {
        if let Some(&(new_state, new_symbol, tape_move)) =
            self.transitions.get(&(self.state, self.tape[self.cursor]))
        {
            self.state = new_state;
            self.tape[self.cursor] = new_symbol;
            if tape_move == TapeMove::Left {
                if self.cursor == 0 {
                    // Move the tape instead of the cursor when at position 0
                    self.tape.push_front(self.blank);
                } else {
                    self.cursor -= 1;
                }
            } else {
                self.cursor += 1;
                if self.cursor == self.tape.len() {
                    self.tape.push_back(self.blank);
                }
            }

            true
        } else {
            false
        }
    }
}

/// Parses a "day 25" turing machine
///  Returns the turing machine and the number of steps to run
fn parse_machine(input: &str) -> (TuringMachine<char, bool>, usize) {
    /// Removes given prefix and returns the second part
    fn trim_prefix<'a>(s: &'a str, prefix: &'_ str) -> Option<&'a str> {
        if s.starts_with(prefix) {
            Some(s.split_at(prefix.len()).1)
        } else {
            None
        }
    }

    /// Parses a state string
    fn parse_state(s: &str) -> char {
        if s.len() == 1 {
            s.chars().next().unwrap()
        } else {
            panic!("invalid state: {}", s)
        }
    }

    /// Parses a symbol string
    fn parse_symbol(s: &str) -> bool {
        if s == "0" {
            false
        } else if s == "1" {
            true
        } else {
            panic!("invalid symbol: {}", s)
        }
    }

    let mut initial_state = None;
    let mut checksum = usize::max_value();

    let mut transitions: TransitionMap<char, bool> = HashMap::new();
    let mut current_state = None;
    let mut current_symbol = None;
    let mut next_symbol = None;
    let mut next_move = None;
    let mut next_state = None;

    for line_untrimmed in input.lines().chain(iter::once("Flush")) {
        // Trim and skip blank lines
        let line = line_untrimmed
            .trim_matches(|c: char| c.is_whitespace() || c == '.' || c == ':' || c == '-');
        if line.is_empty() {
            continue;
        };

        // Handle inner transitions first
        if let (Some(state), Some(symbol)) = (current_state, current_symbol) {
            if let Some(part) = trim_prefix(line, "Write the value ") {
                next_symbol = Some(parse_symbol(part));
                continue;
            } else if let Some(part) = trim_prefix(line, "Move one slot to the ") {
                next_move = Some(part.parse().unwrap());
                continue;
            } else if let Some(part) = trim_prefix(line, "Continue with state ") {
                next_state = Some(parse_state(part));
                continue;
            }

            // Flush current transition
            transitions.insert(
                (state, symbol),
                (
                    next_state.unwrap(),
                    next_symbol.unwrap(),
                    next_move.unwrap(),
                ),
            );
            current_symbol = None;
        }

        // Handle all the other commands
        if let Some(part) = trim_prefix(line, "Begin in state ") {
            initial_state = Some(parse_state(part));
        } else if let Some(part) = trim_prefix(line, "Perform a diagnostic checksum after ") {
            checksum = part.split_whitespace().next().unwrap().parse().unwrap();
        } else if let Some(part) = trim_prefix(line, "In state ") {
            current_state = Some(parse_state(part));
        } else if let Some(part) = trim_prefix(line, "If the current value is ") {
            current_symbol = Some(parse_symbol(part));
        } else if line != "Flush" {
            panic!("malformed input: {}", line);
        }
    }

    (
        TuringMachine::new(false, transitions, initial_state.unwrap(), vec![]),
        checksum,
    )
}

/// Calculate diagnostic checksum
pub fn star1(input: &str) -> String {
    let (mut machine, steps) = parse_machine(input);
    for _ in 0..steps {
        if !machine.step() {
            panic!("machine stopped ?!");
        }
    }

    // Count ones
    machine.tape.iter().filter(|&v| *v).count().to_string()
}
