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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "33");
    star_test!(me1, star1, ME, "1092");

    star_test!(example1b, star2, IN1, "3472");
    star_test!(me2, star2, ME, "3542");

    const IN1: &str = indoc! {"
        Blueprint 1:
          Each ore robot costs 4 ore.
          Each clay robot costs 2 ore.
          Each obsidian robot costs 3 ore and 14 clay.
          Each geode robot costs 2 ore and 7 obsidian.

        Blueprint 2:
          Each ore robot costs 2 ore.
          Each clay robot costs 3 ore.
          Each obsidian robot costs 3 ore and 8 clay.
          Each geode robot costs 3 ore and 12 obsidian.
    "};

    const ME: &str = indoc! {"
        Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 20 obsidian.
        Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 2 ore and 15 obsidian.
        Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 3 ore and 8 obsidian.
        Blueprint 4: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 2 ore and 13 obsidian.
        Blueprint 5: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 12 clay. Each geode robot costs 3 ore and 15 obsidian.
        Blueprint 6: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 2 ore and 16 obsidian.
        Blueprint 7: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 15 obsidian.
        Blueprint 8: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 2 ore and 10 obsidian.
        Blueprint 9: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 9 obsidian.
        Blueprint 10: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 8 obsidian.
        Blueprint 11: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 12 obsidian.
        Blueprint 12: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 4 ore and 20 obsidian.
        Blueprint 13: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 2 ore and 11 obsidian.
        Blueprint 14: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 2 ore and 8 obsidian.
        Blueprint 15: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 10 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 16: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 9 clay. Each geode robot costs 2 ore and 20 obsidian.
        Blueprint 17: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 2 ore and 13 obsidian.
        Blueprint 18: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 4 ore and 16 obsidian.
        Blueprint 19: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 4 ore and 13 obsidian.
        Blueprint 20: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 17 obsidian.
        Blueprint 21: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 3 ore and 19 obsidian.
        Blueprint 22: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 7 clay. Each geode robot costs 2 ore and 16 obsidian.
        Blueprint 23: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 3 ore and 17 obsidian.
        Blueprint 24: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 2 ore and 20 obsidian.
        Blueprint 25: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 3 ore and 16 obsidian.
        Blueprint 26: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 5 clay. Each geode robot costs 3 ore and 18 obsidian.
        Blueprint 27: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.
        Blueprint 28: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 15 clay. Each geode robot costs 2 ore and 20 obsidian.
        Blueprint 29: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 6 clay. Each geode robot costs 2 ore and 10 obsidian.
        Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 7 clay. Each geode robot costs 3 ore and 9 obsidian.
    "};
}
