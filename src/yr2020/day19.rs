#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Rule {
    Terminal(u8),
    Alias(usize),
    Concat(usize, usize),
    Choice(usize, usize),
}

fn push_rule(rules: &mut Vec<Rule>, rule: Rule) -> usize {
    if let Rule::Alias(other) = rule {
        other
    } else {
        rules.push(rule);
        rules.len() - 1
    }
}

fn parse_rule(rules: &mut Vec<Rule>, s: &str) -> Rule {
    if s.starts_with('"') {
        Rule::Terminal(s.as_bytes()[1])
    } else if let Some((a, b)) = s.split_once('|') {
        let left = parse_rule(rules, a.trim());
        let right = parse_rule(rules, b.trim());
        Rule::Choice(push_rule(rules, left), push_rule(rules, right))
    } else if let Some((a, b)) = s.split_once(' ') {
        let right = parse_rule(rules, b.trim());
        Rule::Concat(a.parse().unwrap(), push_rule(rules, right))
    } else {
        Rule::Alias(s.parse().unwrap())
    }
}

fn parse_rules(rules_str: &str) -> Vec<Rule> {
    let mut rules = vec![Rule::Alias(0); 150];
    for rule in rules_str.lines() {
        let (num_str, rest) = rule.split_once(':').unwrap();
        let id: usize = num_str.parse().unwrap();
        rules[id] = parse_rule(&mut rules, rest.trim());
    }
    rules
}

fn append_dedup<T: Ord>(v: &mut Vec<T>, iter: impl IntoIterator<Item = T>) {
    for value in iter {
        if !v.contains(&value) {
            v.push(value);
        }
    }
}

fn matches(rules: &[Rule], id: usize, s: &[u8]) -> Vec<usize> {
    match rules[id] {
        Rule::Terminal(c) if s.first() == Some(&c) => vec![1],
        Rule::Terminal(_) => Vec::new(),
        Rule::Alias(a) => matches(rules, a, s),
        Rule::Concat(a, b) => {
            let mut offs = Vec::new();
            for off_a in matches(rules, a, s) {
                append_dedup(
                    &mut offs,
                    matches(rules, b, &s[off_a..])
                        .into_iter()
                        .map(move |o| o + off_a),
                );
            }
            offs
        }
        Rule::Choice(a, b) => {
            let mut offs = matches(rules, a, s);
            append_dedup(&mut offs, matches(rules, b, s));
            offs
        }
    }
}

fn count_matches(rules: &[Rule], msgs: &str) -> String {
    msgs.lines()
        .filter(|&line| matches(rules, 0, line.as_bytes()).contains(&line.len()))
        .count()
        .to_string()
}

pub fn star1(input: &str) -> String {
    let (rules_str, msgs) = input.split_once("\n\n").unwrap();
    count_matches(&parse_rules(rules_str), msgs)
}

pub fn star2(input: &str) -> String {
    let (rules_str, msgs) = input.split_once("\n\n").unwrap();
    let mut rules = parse_rules(rules_str);

    rules.push(Rule::Concat(42, 8));
    rules[8] = Rule::Choice(42, rules.len() - 1);
    rules.push(Rule::Concat(11, 31));
    rules.push(Rule::Concat(42, rules.len() - 1));
    rules.push(Rule::Concat(42, 31));
    rules[11] = Rule::Choice(rules.len() - 1, rules.len() - 2);

    count_matches(&rules, msgs)
}
