use itertools::izip;

pub fn star1(input: &str) -> String {
    let mut lines = input.lines().rev();
    let is_mult: Vec<bool> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s == "*")
        .collect();
    lines
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect()
        })
        .reduce(|mut a: Vec<u64>, b| {
            for (l, r, &m) in izip!(&mut a, b, &is_mult) {
                if m {
                    *l *= r;
                } else {
                    *l += r;
                }
            }
            a
        })
        .expect("empty input")
        .into_iter()
        .sum::<u64>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    (0..lines[0].len())
        .map(|i| {
            let num_raw: Vec<u8> = lines[..lines.len() - 1]
                .iter()
                .map(|&line| line[i])
                .collect();
            let num_str = str::from_utf8(&num_raw).unwrap().trim();
            let last_char = *lines[lines.len() - 1].get(i).unwrap_or(&b' ');

            if num_str.is_empty() {
                assert_eq!(last_char, b' ');
                None
            } else {
                Some((num_str.parse::<u64>().unwrap(), last_char))
            }
        })
        .fold((0u64, None), |(total, mult_running), val| {
            match (val, mult_running) {
                (Some((v, b'+')), _) | (Some((v, b' ')), None) => (total + v, None),
                (Some((v, b'*' | b' ')), Some(m)) => (total + m * v - m, Some(m * v)),
                (Some((v, b'*')), None) => (total + v, Some(v)),
                (None, _) => (total, None),
                _ => panic!("invalid operator"),
            }
        })
        .0
        .to_string()
}
