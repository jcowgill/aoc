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

#[cfg(test)]
mod test {
    use super::*;

    star_test!(example1a, star1, "0,3,6", "436");
    star_test!(example2a, star1, "1,3,2", "1");
    star_test!(example3a, star1, "2,1,3", "10");
    star_test!(example4a, star1, "1,2,3", "27");
    star_test!(example5a, star1, "2,3,1", "78");
    star_test!(example6a, star1, "3,2,1", "438");
    star_test!(example7a, star1, "3,1,2", "1836");
    star_test!(me1, star1, "6,3,15,13,1,0", "700");

    star_test!(example1b, star2, "0,3,6", "175594");
    star_test!(example2b, star2, "1,3,2", "2578");
    star_test!(example3b, star2, "2,1,3", "3544142");
    star_test!(example4b, star2, "1,2,3", "261214");
    star_test!(example5b, star2, "2,3,1", "6895259");
    star_test!(example6b, star2, "3,2,1", "18");
    star_test!(example7b, star2, "3,1,2", "362");
    star_test!(me2, star2, "6,3,15,13,1,0", "51358");
}
