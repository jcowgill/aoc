fn parse_input(input: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums.last().unwrap() + 3);
    nums
}

pub fn star1(input: &str) -> String {
    let nums = parse_input(input);
    let mut jumps = [0, 0, 0];
    for w in nums.windows(2) {
        jumps[(w[1] - w[0] - 1) as usize] += 1;
    }

    (jumps[0] * jumps[2]).to_string()
}

pub fn star2(input: &str) -> String {
    let nums = parse_input(input);
    let mut arrangements = vec![1u64; nums.len()];

    for (i, n1) in nums[..nums.len() - 2].iter().enumerate().rev() {
        arrangements[i] = arrangements[i + 1];
        if nums[i + 2] - n1 <= 3 {
            arrangements[i] += arrangements[i + 2];
        }
        if i + 3 < nums.len() && nums[i + 3] - n1 <= 3 {
            arrangements[i] += arrangements[i + 3];
        }
    }

    arrangements[0].to_string()
}
