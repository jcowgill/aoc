use itertools::Itertools;
use std::collections::HashMap;

/// Current state of a single game
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct GameState {
    score: [u64; 2],
    pos: [u64; 2],
}

fn parse_input(input: &str) -> GameState {
    GameState {
        score: [0; 2],
        pos: input
            .lines()
            .take(2)
            .map(|l| {
                l.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
                    - 1
            })
            .collect_vec()
            .try_into()
            .unwrap(),
    }
}

pub fn star1(input: &str) -> String {
    let mut state = parse_input(input);
    let mut three_rolls = (1..=100)
        .cycle()
        .enumerate()
        .tuples()
        .map(|((i, a), (_, b), (_, c))| (i, a + b + c));
    let mut player = 0;

    while state.score[0] < 1000 && state.score[1] < 1000 {
        let roll = three_rolls.next().unwrap().1;
        state.pos[player] = (state.pos[player] + roll) % 10;
        state.score[player] += state.pos[player] + 1;
        player = 1 - player;
    }

    (state.score.iter().min().unwrap() * three_rolls.next().unwrap().0 as u64).to_string()
}

pub fn star2(input: &str) -> String {
    // Distribution of roll frequencies from 3 dice starting at index 3
    let dice_dist = [1, 3, 6, 7, 6, 3, 1];

    // Map possible GameStates to the number of universes in that state
    let mut states = HashMap::from([(parse_input(input), 1)]);

    // Frequency of game wins per turn
    let mut win_freq = [0_u64; 2];

    // Iterate while there are active games
    while !states.is_empty() {
        for (player, my_win_freq) in win_freq.iter_mut().enumerate() {
            let mut new_states = HashMap::new();

            for (state, state_freq) in states {
                for (roll_m3, &roll_freq) in dice_dist.iter().enumerate() {
                    let new_freq = state_freq * roll_freq;
                    let mut new_state = state;
                    new_state.pos[player] = (state.pos[player] + roll_m3 as u64 + 3) % 10;
                    new_state.score[player] += new_state.pos[player] + 1;

                    if new_state.score[player] >= 21 {
                        // These universes win this turn
                        *my_win_freq += new_freq;
                    } else {
                        *new_states.entry(new_state).or_insert(0) += new_freq;
                    }
                }
            }

            states = new_states;
        }
    }

    win_freq.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "739785");
    star_test!(me1, star1, ME, "995904");

    star_test!(example1b, star2, IN1, "444356092776315");
    star_test!(me2, star2, ME, "193753136998081");

    const IN1: &str = indoc! {"
        Player 1 starting position: 4
        Player 2 starting position: 8
    "};

    const ME: &str = indoc! {"
        Player 1 starting position: 3
        Player 2 starting position: 4
    "};
}
