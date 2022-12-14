fn star(input: &str, n: usize) -> String {
    // Invariant: all characters in buffer are unique
    let mut buffer = Vec::with_capacity(n);

    for (i, c) in input.chars().enumerate() {
        // Is this character unique?
        if let Some(pos) = buffer.iter().position(|&b| b == c) {
            buffer.drain(..=pos);
        }

        buffer.push(c);

        if buffer.len() >= n {
            return (i + 1).to_string();
        }
    }

    panic!("could not find sequence");
}

pub fn star1(input: &str) -> String {
    star(input, 4)
}

pub fn star2(input: &str) -> String {
    star(input, 14)
}
