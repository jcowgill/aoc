use std::ops::{Add, Mul};

#[derive(Clone, Debug, Eq, PartialEq)]
struct MonkeyInfo {
    items: Vec<u32>,
    op_mul: bool,
    op_param: Option<u32>,
    div_test: u32,
    div_true: usize,
    div_false: usize,
}

fn parse_input(input: &str) -> Vec<MonkeyInfo> {
    input
        .split("\n\n")
        .map(|m| {
            let lines: Vec<_> = m.lines().map(|l| l.trim()).collect();
            let last_int: Vec<_> = lines
                .iter()
                .map(|l| {
                    l.split_ascii_whitespace()
                        .last()
                        .and_then(|p| p.parse().ok())
                })
                .collect();

            MonkeyInfo {
                items: lines[1]
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(|i| i.parse().unwrap())
                    .collect(),
                op_mul: lines[2].starts_with("Operation: new = old *"),
                op_param: last_int[2],
                div_test: last_int[3].unwrap(),
                div_true: last_int[4].unwrap() as usize,
                div_false: last_int[5].unwrap() as usize,
            }
        })
        .collect()
}

fn simulate(input: &str, rounds: usize, item_div: u64) -> String {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0u64; monkeys.len()];
    let modulo = monkeys
        .iter()
        .map(|m| u64::from(m.div_test))
        .product::<u64>();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.split_off(0) {
                let b = monkeys[i].op_param.unwrap_or(item);
                let op = if monkeys[i].op_mul {
                    Mul::mul
                } else {
                    Add::add
                };
                let new = (op(u64::from(item), u64::from(b)) / item_div % modulo) as u32;
                let dest = if new.is_multiple_of(monkeys[i].div_test) {
                    monkeys[i].div_true
                } else {
                    monkeys[i].div_false
                };
                monkeys[dest].items.push(new);
                inspections[i] += 1;
            }
        }
    }

    inspections.sort_unstable();
    inspections.reverse();
    (inspections[0] * inspections[1]).to_string()
}

pub fn star1(input: &str) -> String {
    simulate(input, 20, 3)
}

pub fn star2(input: &str) -> String {
    simulate(input, 10000, 1)
}
