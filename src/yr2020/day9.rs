use std::collections::VecDeque;

fn parse_input(input: &str) -> (usize, Vec<u64>) {
    let (preamble_len, rem) = if let Some(s) = input.strip_prefix("# ") {
        let (a, b) = s.split_once('\n').unwrap();
        (a.parse().unwrap(), b)
    } else {
        (25, input)
    };

    (
        preamble_len,
        rem.lines().map(|l| l.parse::<u64>().unwrap()).collect(),
    )
}

fn invalid_number(preamble_len: usize, nums: &[u64]) -> u64 {
    let mut queue = VecDeque::with_capacity(preamble_len);
    for &num in nums {
        if queue.len() >= preamble_len {
            if queue
                .iter()
                .all(|&d| d > num || !queue.contains(&(num - d)))
            {
                return num;
            }
            queue.pop_front();
        }
        queue.push_back(num);
    }

    panic!("no invalid numbers found")
}

pub fn star1(input: &str) -> String {
    let (preamble_len, nums) = parse_input(input);
    invalid_number(preamble_len, &nums).to_string()
}

pub fn star2(input: &str) -> String {
    let (preamble_len, nums) = parse_input(input);
    let invalid = invalid_number(preamble_len, &nums);

    for wsize in 2..input.len() {
        for w in nums.windows(wsize) {
            if w.iter().sum::<u64>() == invalid {
                return (w.iter().min().unwrap() + w.iter().max().unwrap()).to_string();
            }
        }
    }

    panic!("no solution")
}
