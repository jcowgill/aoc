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
