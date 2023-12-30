use std::{borrow::Cow, collections::HashMap};

type Cache = HashMap<(bool, usize, Option<u32>, usize), u64>;

fn solve_part(cache: &mut Cache, springs: &[u8], live_group: Option<u32>, groups: &[u32]) -> u64 {
    let mut int_solve = |damaged: bool, ss: &[u8]| {
        let key = (damaged, ss.len(), live_group, groups.len());
        if let Some(&v) = cache.get(&key) {
            v
        } else {
            let result = match (damaged, live_group) {
                (false, Some(0) | None) => solve_part(cache, ss, None, groups),
                (true, None) if !groups.is_empty() => {
                    solve_part(cache, ss, Some(groups[0] - 1), &groups[1..])
                }
                (true, Some(l)) if l > 0 => solve_part(cache, ss, Some(l - 1), groups),
                _ => 0,
            };
            cache.insert(key, result);
            result
        }
    };

    match springs.split_first() {
        Some((&b'.', ss)) => int_solve(false, ss),
        Some((&b'#', ss)) => int_solve(true, ss),
        Some((&b'?', ss)) => int_solve(false, ss) + int_solve(true, ss),
        None => u64::from(groups.is_empty() && (live_group.is_none() || live_group == Some(0))),
        _ => panic!("invalid character"),
    }
}

fn solve(
    input: &str,
    s_map: impl Fn(&str) -> Cow<str>,
    g_map: impl Fn(Vec<u32>) -> Vec<u32>,
) -> String {
    input
        .lines()
        .map(|line| {
            let (springs_str, groups_str) = line.split_once(' ').unwrap();
            let springs = s_map(springs_str);
            let groups = g_map(groups_str.split(',').map(|g| g.parse().unwrap()).collect());
            solve_part(&mut HashMap::new(), springs.as_bytes(), None, &groups)
        })
        .sum::<u64>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    solve(input, |s| Cow::Borrowed(s), |g| g)
}

pub fn star2(input: &str) -> String {
    solve(
        input,
        |s| Cow::Owned(format!("{s}?{s}?{s}?{s}?{s}")),
        |g| [&g[..]; 5].concat(),
    )
}
