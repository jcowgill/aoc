fn star(input: &str, move_as_one: bool) -> String {
    let mut lines = input.lines().peekable();
    let mut stacks = Vec::new();

    while let Some(line) = lines.next_if(|l| !l.is_empty()) {
        for (i, c) in line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphabetic())
        {
            if stacks.len() <= i / 4 {
                stacks.resize(i / 4 + 1, Vec::new());
            }

            stacks[i / 4].push(c);
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    for cmd in lines.filter(|l| !l.is_empty()) {
        let parts: Vec<_> = cmd.split_ascii_whitespace().collect();
        let count: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse().unwrap();
        let to: usize = parts[5].parse().unwrap();

        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }

        if move_as_one {
            let stack_len = stacks[to - 1].len();
            stacks[to - 1][stack_len - count..].reverse();
        }
    }

    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

pub fn star1(input: &str) -> String {
    star(input, false)
}

pub fn star2(input: &str) -> String {
    star(input, true)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "CMZ");
    star_test!(me1, star1, ME, "FCVRLMVQP");

    star_test!(example1b, star2, IN1, "MCD");
    star_test!(me2, star2, ME, "RWLWGJGFD");

    const IN1: &str = indoc! {"
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    const ME: &str = indoc! {"
            [M]             [Z]     [V]
            [Z]     [P]     [L]     [Z] [J]
        [S] [D]     [W]     [W]     [H] [Q]
        [P] [V] [N] [D]     [P]     [C] [V]
        [H] [B] [J] [V] [B] [M]     [N] [P]
        [V] [F] [L] [Z] [C] [S] [P] [S] [G]
        [F] [J] [M] [G] [R] [R] [H] [R] [L]
        [G] [G] [G] [N] [V] [V] [T] [Q] [F]
         1   2   3   4   5   6   7   8   9

        move 6 from 9 to 3
        move 2 from 2 to 1
        move 1 from 8 to 2
        move 3 from 7 to 2
        move 7 from 6 to 9
        move 1 from 9 to 5
        move 3 from 5 to 7
        move 6 from 8 to 6
        move 1 from 7 to 8
        move 6 from 6 to 5
        move 4 from 5 to 8
        move 9 from 2 to 9
        move 1 from 2 to 3
        move 3 from 1 to 3
        move 3 from 5 to 1
        move 10 from 3 to 5
        move 4 from 4 to 6
        move 2 from 7 to 6
        move 2 from 6 to 9
        move 6 from 8 to 6
        move 1 from 4 to 3
        move 1 from 4 to 5
        move 1 from 4 to 1
        move 2 from 3 to 1
        move 1 from 3 to 7
        move 8 from 1 to 9
        move 1 from 1 to 2
        move 1 from 2 to 7
        move 6 from 6 to 3
        move 7 from 3 to 5
        move 14 from 5 to 6
        move 2 from 1 to 3
        move 5 from 5 to 8
        move 5 from 8 to 1
        move 2 from 7 to 1
        move 5 from 6 to 9
        move 8 from 9 to 3
        move 13 from 9 to 3
        move 7 from 1 to 4
        move 6 from 4 to 1
        move 22 from 3 to 1
        move 1 from 9 to 3
        move 2 from 6 to 1
        move 1 from 3 to 4
        move 7 from 9 to 8
        move 2 from 1 to 7
        move 2 from 3 to 2
        move 2 from 6 to 9
        move 2 from 7 to 8
        move 1 from 3 to 6
        move 9 from 8 to 6
        move 1 from 2 to 4
        move 8 from 1 to 2
        move 1 from 9 to 4
        move 3 from 4 to 1
        move 1 from 4 to 6
        move 10 from 6 to 5
        move 5 from 2 to 9
        move 6 from 9 to 3
        move 2 from 5 to 3
        move 2 from 9 to 7
        move 7 from 5 to 8
        move 5 from 6 to 2
        move 3 from 3 to 7
        move 3 from 3 to 5
        move 4 from 5 to 8
        move 1 from 3 to 5
        move 6 from 6 to 8
        move 1 from 5 to 7
        move 9 from 8 to 9
        move 1 from 3 to 1
        move 7 from 2 to 7
        move 9 from 7 to 6
        move 2 from 2 to 3
        move 7 from 9 to 3
        move 9 from 6 to 8
        move 7 from 3 to 4
        move 2 from 7 to 6
        move 4 from 4 to 5
        move 3 from 5 to 6
        move 2 from 7 to 4
        move 5 from 4 to 7
        move 13 from 8 to 4
        move 2 from 9 to 4
        move 2 from 8 to 7
        move 6 from 7 to 5
        move 6 from 4 to 2
        move 1 from 7 to 5
        move 3 from 2 to 7
        move 1 from 7 to 8
        move 3 from 2 to 4
        move 2 from 3 to 9
        move 2 from 7 to 2
        move 6 from 5 to 4
        move 3 from 6 to 2
        move 2 from 6 to 9
        move 5 from 2 to 9
        move 12 from 4 to 8
        move 3 from 9 to 2
        move 12 from 1 to 5
        move 4 from 4 to 6
        move 12 from 8 to 9
        move 2 from 6 to 5
        move 1 from 4 to 8
        move 1 from 4 to 1
        move 3 from 2 to 1
        move 2 from 6 to 7
        move 1 from 5 to 9
        move 2 from 1 to 4
        move 10 from 5 to 1
        move 2 from 7 to 3
        move 18 from 9 to 7
        move 8 from 7 to 2
        move 1 from 9 to 6
        move 1 from 6 to 7
        move 10 from 7 to 9
        move 1 from 4 to 2
        move 19 from 1 to 5
        move 8 from 5 to 9
        move 3 from 8 to 4
        move 2 from 5 to 2
        move 2 from 3 to 6
        move 10 from 5 to 2
        move 4 from 1 to 2
        move 2 from 9 to 2
        move 1 from 1 to 6
        move 2 from 5 to 6
        move 1 from 8 to 7
        move 1 from 5 to 8
        move 1 from 5 to 6
        move 18 from 2 to 5
        move 5 from 2 to 1
        move 6 from 5 to 8
        move 1 from 8 to 9
        move 2 from 2 to 4
        move 1 from 2 to 6
        move 2 from 7 to 6
        move 1 from 4 to 1
        move 4 from 8 to 5
        move 1 from 2 to 9
        move 2 from 8 to 3
        move 1 from 3 to 6
        move 1 from 4 to 8
        move 1 from 8 to 9
        move 10 from 5 to 7
        move 5 from 5 to 1
        move 2 from 4 to 1
        move 3 from 7 to 6
        move 12 from 1 to 4
        move 8 from 9 to 5
        move 6 from 7 to 4
        move 1 from 7 to 9
        move 4 from 4 to 3
        move 1 from 1 to 7
        move 3 from 9 to 5
        move 2 from 3 to 1
        move 1 from 7 to 6
        move 8 from 4 to 7
        move 1 from 7 to 6
        move 7 from 6 to 4
        move 2 from 1 to 3
        move 1 from 7 to 1
        move 1 from 3 to 7
        move 1 from 1 to 6
        move 4 from 9 to 3
        move 5 from 4 to 6
        move 12 from 6 to 2
        move 3 from 9 to 4
        move 8 from 2 to 6
        move 2 from 9 to 6
        move 8 from 5 to 6
        move 4 from 5 to 8
        move 14 from 6 to 3
        move 11 from 4 to 9
        move 2 from 2 to 7
        move 8 from 3 to 9
        move 11 from 3 to 6
        move 14 from 9 to 1
        move 7 from 1 to 3
        move 2 from 9 to 5
        move 2 from 2 to 8
        move 6 from 7 to 5
        move 1 from 9 to 8
        move 13 from 6 to 3
        move 4 from 6 to 8
        move 3 from 1 to 6
        move 5 from 5 to 8
        move 7 from 8 to 7
        move 2 from 1 to 8
        move 1 from 4 to 1
        move 4 from 8 to 9
        move 8 from 7 to 5
        move 1 from 8 to 1
        move 4 from 9 to 3
        move 1 from 4 to 5
        move 5 from 5 to 2
        move 1 from 8 to 9
        move 1 from 8 to 6
        move 2 from 6 to 2
        move 4 from 8 to 6
        move 4 from 1 to 8
        move 4 from 8 to 5
        move 1 from 9 to 8
        move 1 from 2 to 3
        move 4 from 6 to 1
        move 1 from 8 to 2
        move 3 from 5 to 4
        move 4 from 2 to 5
        move 1 from 7 to 9
        move 1 from 2 to 6
        move 3 from 1 to 8
        move 2 from 4 to 5
        move 2 from 6 to 1
        move 3 from 8 to 9
        move 4 from 9 to 2
        move 1 from 7 to 1
        move 1 from 6 to 7
        move 4 from 1 to 6
        move 1 from 7 to 4
        move 6 from 2 to 8
        move 2 from 4 to 8
        move 1 from 9 to 5
        move 3 from 6 to 2
        move 1 from 6 to 4
        move 7 from 3 to 5
        move 2 from 8 to 1
        move 3 from 2 to 8
        move 6 from 8 to 5
        move 17 from 5 to 3
        move 2 from 1 to 6
        move 3 from 8 to 3
        move 1 from 9 to 5
        move 11 from 5 to 2
        move 40 from 3 to 5
        move 11 from 2 to 7
        move 4 from 7 to 8
        move 1 from 8 to 9
        move 1 from 3 to 5
        move 1 from 4 to 8
        move 19 from 5 to 8
        move 7 from 7 to 8
        move 16 from 5 to 2
        move 6 from 5 to 8
        move 1 from 5 to 8
        move 1 from 9 to 4
        move 1 from 6 to 1
        move 1 from 4 to 7
        move 1 from 6 to 9
        move 1 from 1 to 7
        move 1 from 7 to 3
        move 1 from 7 to 2
        move 1 from 9 to 8
        move 1 from 3 to 4
        move 1 from 4 to 6
        move 14 from 2 to 9
        move 24 from 8 to 4
        move 8 from 8 to 3
        move 1 from 6 to 3
        move 16 from 4 to 1
        move 3 from 8 to 4
        move 3 from 3 to 8
        move 4 from 3 to 4
        move 1 from 3 to 9
        move 13 from 9 to 4
        move 16 from 1 to 8
        move 8 from 8 to 1
        move 3 from 1 to 7
        move 1 from 8 to 6
        move 1 from 3 to 8
        move 10 from 8 to 5
        move 5 from 5 to 2
        move 3 from 8 to 9
        move 1 from 8 to 9
        move 1 from 4 to 5
        move 5 from 2 to 6
        move 3 from 5 to 2
        move 1 from 6 to 1
        move 5 from 1 to 5
        move 1 from 1 to 5
        move 2 from 7 to 3
        move 2 from 3 to 2
        move 1 from 5 to 7
        move 7 from 5 to 3
        move 5 from 9 to 5
        move 2 from 7 to 9
        move 4 from 5 to 6
        move 2 from 9 to 8
        move 2 from 2 to 4
        move 5 from 3 to 5
        move 1 from 3 to 2
        move 7 from 4 to 9
        move 1 from 8 to 1
        move 1 from 2 to 1
        move 9 from 4 to 6
        move 2 from 1 to 8
        move 1 from 3 to 9
        move 2 from 8 to 6
        move 13 from 4 to 6
        move 1 from 8 to 7
        move 2 from 9 to 6
        move 3 from 5 to 7
        move 3 from 2 to 5
        move 3 from 2 to 6
        move 5 from 6 to 2
        move 4 from 2 to 5
        move 4 from 5 to 7
        move 5 from 5 to 7
        move 7 from 9 to 6
        move 6 from 7 to 2
        move 22 from 6 to 5
        move 10 from 5 to 8
        move 7 from 5 to 4
        move 8 from 8 to 5
        move 18 from 6 to 2
        move 5 from 7 to 5
        move 1 from 8 to 2
        move 6 from 5 to 1
        move 7 from 4 to 2
        move 4 from 1 to 5
        move 1 from 7 to 9
        move 1 from 8 to 6
        move 1 from 7 to 8
        move 10 from 5 to 9
        move 12 from 2 to 1
        move 8 from 5 to 2
        move 19 from 2 to 9
        move 1 from 6 to 8
        move 13 from 9 to 3
        move 8 from 1 to 2
        move 5 from 1 to 3
        move 10 from 2 to 1
        move 7 from 2 to 5
        move 3 from 5 to 7
        move 4 from 1 to 3
        move 1 from 2 to 3
        move 3 from 1 to 2
        move 1 from 8 to 6
        move 2 from 7 to 5
        move 4 from 1 to 3
        move 6 from 5 to 4
        move 2 from 2 to 1
        move 1 from 2 to 9
        move 6 from 4 to 5
        move 5 from 5 to 9
        move 1 from 6 to 8
        move 1 from 5 to 1
        move 6 from 9 to 2
        move 5 from 2 to 4
        move 3 from 1 to 6
        move 2 from 4 to 7
        move 22 from 3 to 9
        move 1 from 8 to 4
        move 2 from 4 to 3
        move 2 from 6 to 1
        move 2 from 1 to 5
        move 1 from 6 to 7
        move 1 from 7 to 4
        move 6 from 3 to 7
        move 1 from 2 to 4
        move 8 from 7 to 3
        move 1 from 4 to 5
        move 1 from 7 to 9
        move 5 from 3 to 6
        move 1 from 8 to 4
        move 4 from 3 to 2
        move 32 from 9 to 3
        move 3 from 6 to 7
        move 5 from 9 to 3
        move 1 from 9 to 7
        move 2 from 9 to 2
        move 2 from 4 to 3
        move 2 from 5 to 4
        move 5 from 3 to 2
        move 3 from 7 to 8
        move 1 from 7 to 2
        move 1 from 8 to 5
        move 1 from 3 to 4
        move 5 from 4 to 5
        move 4 from 5 to 2
        move 3 from 5 to 7
        move 1 from 7 to 5
        move 1 from 6 to 5
        move 2 from 8 to 5
        move 15 from 2 to 4
        move 3 from 5 to 6
        move 4 from 6 to 5
        move 2 from 5 to 2
        move 1 from 2 to 4
        move 25 from 3 to 9
        move 2 from 5 to 2
        move 11 from 9 to 2
        move 13 from 2 to 1
        move 4 from 4 to 7
        move 12 from 9 to 8
        move 6 from 7 to 8
        move 7 from 4 to 7
        move 7 from 7 to 8
        move 1 from 5 to 1
        move 5 from 4 to 3
        move 2 from 2 to 1
        move 2 from 9 to 5
        move 7 from 1 to 7
        move 1 from 1 to 4
        move 12 from 3 to 2
        move 1 from 3 to 9
        move 1 from 1 to 3
        move 1 from 9 to 1
        move 7 from 7 to 2
        move 1 from 4 to 7
        move 2 from 8 to 7
        move 7 from 1 to 2
        move 1 from 3 to 4
        move 26 from 2 to 1
        move 4 from 8 to 1
        move 3 from 1 to 6
        move 1 from 6 to 3
        move 1 from 6 to 9
        move 1 from 3 to 8
        move 20 from 1 to 3
        move 1 from 9 to 7
        move 4 from 7 to 1
        move 1 from 5 to 3
        move 4 from 3 to 5
        move 1 from 6 to 2
        move 6 from 3 to 2
        move 8 from 1 to 4
        move 1 from 1 to 5
        move 3 from 1 to 4
        move 7 from 2 to 4
        move 10 from 3 to 8
        move 4 from 4 to 3
        move 12 from 4 to 7
        move 3 from 3 to 1
        move 2 from 4 to 3
        move 2 from 8 to 1
        move 6 from 8 to 9
        move 5 from 9 to 6
        move 1 from 9 to 3
        move 3 from 8 to 7
        move 10 from 8 to 5
        move 4 from 8 to 7
        move 9 from 7 to 9
        move 4 from 8 to 4
        move 2 from 4 to 3
        move 3 from 1 to 7
        move 11 from 7 to 4
        move 6 from 4 to 8
        move 1 from 7 to 3
        move 4 from 5 to 1
        move 5 from 3 to 6
        move 5 from 9 to 4
        move 1 from 9 to 8
        move 10 from 4 to 8
        move 5 from 1 to 2
        move 1 from 7 to 6
        move 9 from 6 to 3
        move 7 from 8 to 7
        move 3 from 4 to 1
        move 2 from 2 to 1
        move 9 from 8 to 3
        move 10 from 5 to 8
        move 18 from 3 to 9
        move 1 from 7 to 8
        move 1 from 5 to 3
        move 4 from 8 to 3
        move 2 from 6 to 3
        move 6 from 7 to 2
        move 1 from 5 to 3
        move 1 from 1 to 9
        move 10 from 3 to 9
        move 4 from 1 to 8
        move 13 from 8 to 1
        move 3 from 1 to 8
        move 3 from 2 to 4
        move 5 from 2 to 6
        move 5 from 6 to 4
        move 28 from 9 to 2
        move 2 from 9 to 5
        move 2 from 5 to 2
        move 1 from 3 to 7
        move 2 from 1 to 4
        move 3 from 8 to 3
        move 1 from 9 to 4
        move 3 from 4 to 6
        move 2 from 3 to 7
        move 8 from 1 to 5
        move 3 from 7 to 6
        move 14 from 2 to 8
        move 1 from 9 to 1
        move 6 from 5 to 6
        move 4 from 2 to 5
        move 9 from 8 to 2
        move 4 from 8 to 4
        move 7 from 2 to 4
        move 12 from 4 to 3
        move 5 from 4 to 7
        move 5 from 7 to 4
        move 1 from 8 to 7
        move 1 from 4 to 5
        move 2 from 5 to 4
        move 1 from 5 to 8
        move 1 from 5 to 9
    "};
}
