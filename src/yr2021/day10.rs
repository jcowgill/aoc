fn evaluate_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();

    for c in line.trim().chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            _ if c != stack.pop().unwrap() => return Err(c),
            _ => (),
        }
    }

    Ok(stack)
}

pub fn star1(input: &str) -> String {
    input
        .lines()
        .flat_map(|l| evaluate_line(l).err())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("invalid char {} found", c),
        })
        .sum::<u32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let mut scores: Vec<u64> = input
        .lines()
        .flat_map(evaluate_line)
        .map(|stack| {
            stack.iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2].to_string()
}
