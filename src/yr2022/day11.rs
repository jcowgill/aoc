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
                let dest = if new % monkeys[i].div_test == 0 {
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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "10605");
    star_test!(me1, star1, ME, "58322");

    star_test!(example1b, star2, IN1, "2713310158");
    star_test!(me2, star2, ME, "13937702909");

    const IN1: &str = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    const ME: &str = indoc! {"
        Monkey 0:
          Starting items: 59, 65, 86, 56, 74, 57, 56
          Operation: new = old * 17
          Test: divisible by 3
            If true: throw to monkey 3
            If false: throw to monkey 6

        Monkey 1:
          Starting items: 63, 83, 50, 63, 56
          Operation: new = old + 2
          Test: divisible by 13
            If true: throw to monkey 3
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 93, 79, 74, 55
          Operation: new = old + 1
          Test: divisible by 2
            If true: throw to monkey 0
            If false: throw to monkey 1

        Monkey 3:
          Starting items: 86, 61, 67, 88, 94, 69, 56, 91
          Operation: new = old + 7
          Test: divisible by 11
            If true: throw to monkey 6
            If false: throw to monkey 7

        Monkey 4:
          Starting items: 76, 50, 51
          Operation: new = old * old
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 5

        Monkey 5:
          Starting items: 77, 76
          Operation: new = old + 8
          Test: divisible by 17
            If true: throw to monkey 2
            If false: throw to monkey 1

        Monkey 6:
          Starting items: 74
          Operation: new = old * 2
          Test: divisible by 5
            If true: throw to monkey 4
            If false: throw to monkey 7

        Monkey 7:
          Starting items: 86, 85, 52, 86, 91, 95
          Operation: new = old + 6
          Test: divisible by 7
            If true: throw to monkey 4
            If false: throw to monkey 5
    "};
}
