use itertools::Itertools;
use nalgebra::Vector4;
use num::Zero;

fn solve_blueprint<'a>(parts_iter: impl Iterator<Item = &'a str>, initial_time: i32) -> i32 {
    let parts: Vec<_> = parts_iter
        .enumerate()
        .filter(|(i, _)| [1, 6, 12, 18, 21, 27, 30].contains(i))
        .map(|(_, p)| p.trim_matches(':').parse().unwrap())
        .collect();

    let costs = [
        Vector4::new(parts[1], 0, 0, 0),
        Vector4::new(parts[2], 0, 0, 0),
        Vector4::new(parts[3], parts[4], 0, 0),
        Vector4::new(parts[5], 0, parts[6], 0),
    ];

    let mut best_geodes = 0;
    let mut states = vec![(Vector4::new(1, 0, 0, 0), Vector4::zero(), initial_time)];

    while let Some((robots, resources, time)) = states.pop() {
        // Test no more robots
        let idle_geodes = resources[3] + robots[3] * time;
        best_geodes = best_geodes.max(idle_geodes);

        // Check geodes upper bound
        if best_geodes >= idle_geodes + time * (time - 1) / 2 {
            continue;
        }

        for (i, cost) in costs.iter().enumerate() {
            /* Don't build if we already have enough of this robot */
            if i < 3 && costs.iter().all(|c| robots[i] >= c[i]) {
                continue;
            }

            for j in 0..time {
                let start_resources = resources - cost + robots * j;
                if start_resources.iter().all(|&a| a >= 0) {
                    states.push((
                        robots + Vector4::ith(i, 1),
                        start_resources + robots,
                        time - j - 1,
                    ));
                    break;
                }
            }
        }
    }

    best_geodes
}

pub fn star1(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .chunks(32)
        .into_iter()
        .enumerate()
        .map(|(i, b)| (i as i32 + 1) * solve_blueprint(b, 24))
        .sum::<i32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .chunks(32)
        .into_iter()
        .take(3)
        .map(|b| solve_blueprint(b, 32))
        .product::<i32>()
        .to_string()
}
