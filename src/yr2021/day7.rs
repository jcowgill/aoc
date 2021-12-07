pub fn star1(input: &str) -> String {
    let mut positions: Vec<i32> = input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();
    positions.sort_unstable();
    let median = positions[positions.len() / 2];
    positions
        .iter()
        .map(|p| (p - median).abs())
        .sum::<i32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let positions: Vec<i32> = input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();
    let mean1 = positions.iter().sum::<i32>() / (positions.len() as i32);

    // Try mean and mean + 1 as the midpoint in case the mean is not an integer
    (mean1..=mean1 + 1)
        .map(|mean| {
            positions
                .iter()
                .map(|p| (p - mean).abs())
                .map(|r| (r * (r + 1)) / 2)
                .sum::<i32>()
        })
        .min()
        .unwrap()
        .to_string()
}
