/// Prints "spinlock" final value
pub fn star1(input: &str) -> String {
    let step: usize = input.parse().unwrap();
    let mut buffer: Vec<usize> = vec![0];
    let mut pos = 0;

    for i in 1..2018 {
        pos = (pos + step) % i + 1;
        buffer.insert(pos, i);
    }

    buffer[(pos + 1) % buffer.len()].to_string()
}

/// Prints "spinlock" final value at pos 1 after 5 million iterations
pub fn star2(input: &str) -> String {
    // We explot the fact that inserting a new value never affects items before the current
    // position. Therefore to know what is in position 1, we only care about changes that happen
    // when inserting at position 1 (nothing is ever inserted at position 0).
    let step: usize = input.parse().unwrap();
    let mut pos1_value = 1;
    let mut pos = 1;

    // We have already "pre-computed" iteration 1 with initial values above
    for i in 2..50000000 {
        pos = (pos + step) % i + 1;
        if pos == 1 {
            pos1_value = i;
        }
    }

    pos1_value.to_string()
}
