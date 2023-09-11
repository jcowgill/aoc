fn run(input: &str, turns: usize) -> String {
    let first_turns: Vec<usize> = input.split(',').map(|n| n.parse().unwrap()).collect();
    let mut turn_map = vec![u32::MAX; turns];

    let mut process_turn = |i, v| {
        let old_i = std::mem::replace(&mut turn_map[v], i as u32);
        if old_i == u32::MAX {
            0
        } else {
            i - (old_i as usize)
        }
    };

    let mut next = 0;
    for (i, &v) in first_turns.iter().enumerate() {
        next = process_turn(i, v);
    }
    for i in first_turns.len()..(turns - 1) {
        next = process_turn(i, next);
    }
    next.to_string()
}

pub fn star1(input: &str) -> String {
    run(input, 2020)
}

pub fn star2(input: &str) -> String {
    run(input, 30_000_000)
}
