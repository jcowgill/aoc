fn solve(input: &str, rev: bool) -> String {
    input
        .lines()
        .map(|line| {
            let mut nums: Vec<_> = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            if rev {
                nums.reverse();
            }

            (1..nums.len())
                .scan(nums, |state, _| {
                    Some(std::mem::replace(
                        state,
                        state.windows(2).map(|w| w[1] - w[0]).collect(),
                    ))
                })
                .map(|d| *d.last().unwrap())
                .sum::<i32>()
        })
        .sum::<i32>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    solve(input, false)
}

pub fn star2(input: &str) -> String {
    solve(input, true)
}
