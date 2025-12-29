use itertools::Itertools;

type State = [u64; 150];

pub fn solve(input: &str) -> (State, u32) {
    let start_pos = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    let mut splits = 0;
    let mut state = [0; 150];
    state[start_pos] = 1;

    for line in input.lines().skip(1) {
        for split in line.chars().positions(|c| c == '^') {
            if state[split] != 0 {
                state[split - 1] += state[split];
                state[split + 1] += state[split];
                state[split] = 0;
                splits += 1;
            }
        }
    }

    (state, splits)
}

pub fn star1(input: &str) -> String {
    solve(input).1.to_string()
}

pub fn star2(input: &str) -> String {
    solve(input).0.into_iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let a = "....S....\n.........\n....^....\n.........\n";
        assert_eq!(star1(a), "1");
        assert_eq!(star2(a), "2");
    }
}
