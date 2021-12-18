fn simulate_fish(input: &str, days: u32) -> String {
    let mut counts = [0; 9];

    for fish in input.split(',') {
        counts[fish.trim().parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..days {
        counts = [
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[7] + counts[0],
            counts[8],
            counts[0],
        ];
    }

    counts.iter().sum::<u64>().to_string()
}

pub fn star1(input: &str) -> String {
    simulate_fish(input, 80)
}

pub fn star2(input: &str) -> String {
    simulate_fish(input, 256)
}
