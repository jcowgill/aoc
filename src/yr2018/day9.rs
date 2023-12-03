use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

/// Rotates a VecDeque left (move first elements to the back)
fn rotate_left<T>(deque: &mut VecDeque<T>, amount: u32) {
    for _ in 0..amount {
        if let Some(item) = deque.pop_front() {
            deque.push_back(item);
        }
    }
}

/// Rotates a VecDeque right (move last elements to the front)
fn rotate_right<T>(deque: &mut VecDeque<T>, amount: u32) {
    for _ in 0..amount {
        if let Some(item) = deque.pop_back() {
            deque.push_front(item);
        }
    }
}

/// Simulates a marble game returning the scores of each player
fn marble_game(players: u32, last_marble: u32) -> Vec<u32> {
    let mut marbles: VecDeque<u32> = VecDeque::with_capacity(last_marble as usize);
    let mut scores = Vec::new();

    // Initial state
    marbles.push_back(0);
    scores.resize(players as usize, 0);

    // In the main algorithm the current marble is the **last** in the
    // marbles list. This makes the modulo 23 move slower, but all the
    // other moves faster (by 1 rotation)
    for m in 1..(last_marble + 1) {
        if m % 23 == 0 {
            rotate_right(&mut marbles, 7);
            scores[(m % players) as usize] += m + marbles.pop_back().unwrap();
            rotate_left(&mut marbles, 1);
        } else {
            rotate_left(&mut marbles, 1);
            marbles.push_back(m);
        }
    }

    scores
}

/// From an input description, returns the max score as a string
fn max_score(input: &str, multiplier: u32) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("^([0-9]+) players; last marble is worth ([0-9]+) points$").unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let players: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let last_marble: u32 = caps.get(2).unwrap().as_str().parse().unwrap();

    marble_game(players, last_marble * multiplier)
        .iter()
        .max()
        .unwrap()
        .to_string()
}

pub fn star1(input: &str) -> String {
    max_score(input, 1)
}

pub fn star2(input: &str) -> String {
    max_score(input, 100)
}

#[cfg(test)]
mod test {
    use super::*;

    star_test!(
        example1,
        star1,
        "9 players; last marble is worth 25 points",
        "32"
    );
    star_test!(
        example2,
        star1,
        "10 players; last marble is worth 1618 points",
        "8317"
    );
    star_test!(
        example3,
        star1,
        "13 players; last marble is worth 7999 points",
        "146373"
    );
    star_test!(
        example4,
        star1,
        "17 players; last marble is worth 1104 points",
        "2764"
    );
    star_test!(
        example5,
        star1,
        "21 players; last marble is worth 6111 points",
        "54718"
    );
    star_test!(
        example6,
        star1,
        "30 players; last marble is worth 5807 points",
        "37305"
    );
    star_test!(
        me1,
        star1,
        "465 players; last marble is worth 71498 points",
        "383475"
    );

    star_test!(
        me2,
        star2,
        "465 players; last marble is worth 71498 points",
        "3148209772"
    );
}
