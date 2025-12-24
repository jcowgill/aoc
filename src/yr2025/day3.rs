fn solve<const N: usize>(line: &[u8]) -> u64 {
    let mut best = [0; N];
    for &b in line {
        let value = u64::from(b - b'0');
        for i in (0..N).rev() {
            // dp[pos][digits] = max(dp[pos - 1][digits], dp[pos - 1][digits - 1] & line[pos])
            let prev = if i == 0 { 0 } else { best[i - 1] };
            best[i] = best[i].max(prev * 10 + value);
        }
    }
    best[N - 1]
}

fn solve_all<const N: usize>(input: &str) -> String {
    input
        .lines()
        .map(|l| solve::<N>(l.as_bytes()))
        .sum::<u64>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    solve_all::<2>(input)
}

pub fn star2(input: &str) -> String {
    solve_all::<12>(input)
}
