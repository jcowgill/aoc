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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1, star1, IN1, "2");
    star_test!(example2a, star1, IN2, "3");
    star_test!(example3a, star1, IN3, "1");
    star_test!(me1, star1, ME, "190");

    star_test!(example2b, star2, IN2, "12");
    star_test!(example3b, star2, IN3, "5");
    star_test!(me2, star2, ME, "311");

    const IN1: &str = indoc! {r#"
        0: 4 1 5
        1: 2 3 | 3 2
        2: 4 4 | 5 5
        3: 4 5 | 5 4
        4: "a"
        5: "b"

        ababbb
        bababa
        abbbab
        aaabbb
        aaaabbb
    "#};

    const IN2: &str = indoc! {r#"
        42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: "a"
        11: 42 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: "b"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1

        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
    "#};

    const IN3: &str = indoc! {r#"
        0: 8
        8: 42
        42: "a"

        a
        aa
        aaa
        aaaa
        aaaaa
        aaaaab
    "#};

    const ME: &str = indoc! {r#"
        62: 110 112 | 92 68
        85: 38 92 | 102 110
        63: 92 6 | 110 23
        82: 110 | 92
        99: 110 122 | 92 54
        33: 110 61 | 92 103
        24: 110 92 | 110 110
        76: 3 92
        109: 26 92 | 81 110
        94: 92 78 | 110 28
        17: 110 115
        61: 92 110 | 110 82
        25: 9 110 | 61 92
        114: 47 110 | 63 92
        21: 30 82
        103: 82 82
        69: 5 110 | 43 92
        73: 110 116 | 92 69
        52: 92 81 | 110 103
        22: 119 92 | 50 110
        108: 37 92 | 35 110
        2: 71 92 | 20 110
        119: 103 110 | 88 92
        102: 101 92 | 49 110
        92: "b"
        91: 92 54 | 110 3
        64: 92 115 | 110 24
        68: 92 30 | 110 81
        89: 115 110 | 26 92
        80: 61 110 | 81 92
        19: 110 73 | 92 1
        16: 115 92 | 103 110
        15: 123 110 | 58 92
        115: 110 110 | 92 110
        70: 92 30 | 110 9
        40: 110 96 | 92 99
        50: 88 110 | 115 92
        18: 26 92 | 48 110
        81: 92 110
        107: 110 30 | 92 115
        38: 84 110 | 12 92
        44: 40 92 | 66 110
        27: 125 92 | 2 110
        41: 92 128 | 110 118
        56: 81 92 | 30 110
        88: 110 92
        13: 75 92
        55: 75 110 | 26 92
        12: 13 92 | 52 110
        28: 122 92
        121: 75 92 | 24 110
        113: 92 81 | 110 54
        98: 103 110 | 3 92
        6: 92 62 | 110 126
        96: 75 92 | 9 110
        9: 82 92 | 92 110
        0: 8 11
        32: 104 92 | 91 110
        116: 110 32 | 92 67
        104: 92 81 | 110 30
        5: 110 109 | 92 105
        117: 110 22 | 92 59
        128: 92 24 | 110 9
        66: 92 98 | 110 21
        46: 76 92 | 89 110
        36: 92 90 | 110 127
        20: 110 115 | 92 3
        4: 48 110 | 30 92
        26: 92 82 | 110 110
        39: 30 110 | 115 92
        127: 61 110 | 9 92
        84: 110 104 | 92 78
        100: 97 92 | 68 110
        106: 110 7 | 92 18
        123: 110 64 | 92 68
        111: 110 74 | 92 46
        42: 114 110 | 19 92
        58: 28 92 | 16 110
        47: 110 15 | 92 124
        54: 92 92 | 110 92
        51: 110 117 | 92 111
        130: 92 41 | 110 106
        30: 92 92 | 110 110
        72: 17 110 | 109 92
        129: 130 92 | 108 110
        14: 85 110 | 51 92
        53: 87 92 | 44 110
        112: 92 30 | 110 122
        45: 61 82
        105: 75 92 | 61 110
        90: 110 122 | 92 9
        122: 110 92 | 92 110
        83: 110 33 | 92 56
        118: 110 61 | 92 75
        74: 120 92 | 78 110
        29: 110 65 | 92 60
        31: 14 92 | 10 110
        110: "a"
        87: 36 110 | 94 92
        57: 92 48 | 110 81
        78: 92 115 | 110 54
        97: 92 48 | 110 122
        1: 110 27 | 92 34
        95: 92 107 | 110 86
        79: 110 55 | 92 77
        37: 92 109 | 110 25
        126: 4 110 | 39 92
        101: 45 92 | 80 110
        59: 110 70 | 92 105
        7: 61 92 | 48 110
        48: 92 92
        35: 104 92 | 57 110
        120: 48 92 | 48 110
        34: 29 110 | 79 92
        93: 54 92 | 26 110
        71: 115 110 | 61 92
        125: 121 110 | 45 92
        60: 122 92 | 54 110
        86: 54 110 | 9 92
        49: 97 110 | 91 92
        3: 110 110
        23: 110 95 | 92 72
        65: 92 103 | 110 115
        10: 129 110 | 53 92
        8: 42
        11: 42 31
        77: 103 92 | 81 110
        67: 93 92 | 112 110
        75: 92 110 | 92 92
        124: 83 110 | 100 92
        43: 92 113 | 110 56

        abbabbbbaaabbbbabbbababaaabaabaabbaabbba
        babaaaabbaaabbbbbbbbbaba
        bbbbaaaabaaabbabaababbbabaaaaaababaabaaaaabbabbb
        baabbbbbbbaababaababbbabbababbbbaaaabbbbbabaabbabaaababb
        aabbaabbbbbaabbbbaaabaab
        bbaaabbaaaabababbbbaabaa
        abbbbbababbaabaabaaaabbbaaaabaab
        abaabbbbabaaabaabbbbaabbabaabaaabbaaabbbabbaabbbaabbbabb
        baabbbbbaabbaaabbabbabaa
        babbabbbabaabaaaaabbababaaaabaaaabbababbabababbaaababaaa
        aaababaabababbbbabaaaaab
        bbabaabaabbaabaabbbbbbab
        bbbbaaaabaaabbabbbbbbaababaabbaabaabbaab
        aaabbabbbabababbaabbabba
        abbbbbbabbababbbbabbabbabbabaabbaaabbaab
        bbbbabaaabbaabbabaaabbbababababbabaabbaabbabbbaa
        aabaaaaabaabaaabaaabbaababbbaaababbbbbbabbbbabababbbabbaabaaabaa
        baaaababbaaaaaabaabbbabbaaabaaab
        abbabbbaaabaaabbaaaaabbb
        ababaaaababaaaaaaababbaa
        babbabbabbababababbaaabb
        bbabbaabaaabbabbababaaababababaaaaababaabbaababb
        bbababbbbbabaaabaaabaaaa
        abbaaabaaaabaabbbbbaaaba
        aaaabababbbbabaabaabaaabbabbabbabaaababaaabaabbaabaaabbabbaaaabababbbaab
        aabaababaaaaaabbaabaaabbbabbbaaabaabaaaaabbaaaba
        bababaaaabbaabbbaaaabbab
        abababaabbbbbaababbaaaabbabaabbbabbbbaaaabaaabbbabbbbaba
        ababababababbbaaababbbba
        ababbabbabaabbbaabaaaabbaaaaaaaaaaabaaaabbbababb
        aaabbabbbaaabaaaabaabaaa
        abbbbabbabbbbaaabbbbabbbbabaaaababbbabaababbaabbabaaabba
        bbbababaaaaabaaabaabaaabbaababba
        aaaabbbbbbabababbbaaabab
        aabaababbbbbaabaabaaabaaabababbbbababbabbbbabbaa
        babbabbaabababaaabbbabaa
        baaaaaaabbabababbbaabaaa
        aaababaaaaabaaaaabaaabbaabaaaaab
        abaaabaabaaabaabaaabaaaabbaabaaabbbbbbab
        aaabbbabaababbbbbaabaaba
        bababbbaabababaabababaab
        bababbbaaaabaaaabbbaaabbabbaaabb
        ababaaabbababaaabbabbbbabaabaabaabaababb
        bbabbaabbabaabbbabbabbbaaabbaabaaaaaaaababbbbaaaabbaabab
        aaababababaaaabbaaaaabba
        babaababbbbabababaabaaaaababbabaabaaabba
        aaaabaaaabababaaaaabbaaa
        aabaaaaababbbaaaabbbabababbabbaa
        bbbbaababaabbbabbbbabbaa
        baaaaaabaabaaaaaabbaabbaabbabbbbbbbabbabbbbbabbbabaabbab
        ababababbaaaaaabaabaabbb
        abbbababababbbaababbaabb
        aabbaaabaabbaabbaaaaaabbbbaaabbaabbbaaaa
        bbabbbbaaababaababbbbbbaabbbbbabbabbabbb
        babbbabaabaabbbabaabbaab
        bbabbbabababbbabaababbbaaabaaaababbbbaba
        babbaaaabbbabbbbabbaaaba
        bbbbababbbababbbbbabbbabbbaabbbbaaabbaaa
        ababbbaababbbabaaaaaaabaabaaaaaabbabaabbaaaaaabababaaabaaaaaababbbbabbbbaababbaabbbbaaab
        aabbbabbaabbaabbbaaabababaaabaabbbabbaba
        bbbabaabbbabbaabbaabaaaababbababababbbba
        abaabbbabaaaaabbabaaabbb
        bababbabaabaabaaaabbbbaabbababba
        aabbaaabaabbbabbaababaaa
        baababbbbbabaabaaaaabbab
        bbbbabbaabbbbbbbabbbaaaaaaabaabababaaaaaabababbbbaababab
        baaaabbabbaaaaaababaaaaaabaabbbbababbbbb
        baababbbabbbbaaabbaabaaa
        bbbabbabbbbbabbbbaaaaaaaaaabbabaabaababb
        bbbbbbaabaabbbaaabbbaaba
        bbbababaaabbbbbbbbbbababaabaabaababbbbba
        abbaabbaabbaaaababaaaaababaabaaaaabbbaabaabaaaabaabbabbbbabaabba
        baaaabbbaaabbaabaabbbabababbaaab
        abaaabaaabababaababbbbbb
        bbbbaaaabbabaabaabaaabaabaabbaaa
        bbaaabbbbaaaaaaabaabbbbbbbbaabbbaaaaabbb
        abbaabbabaaabbabaaabbbbabbababba
        baabaaaaaabbabaaaabaabbbbaabbaab
        babbbababaaababaabaabbaa
        aabbbbaaababbbaaabaaaaab
        bbaaabbbaabaaabaabbbabba
        babbbaaabababaaababbbaab
        aaaaaaaaabbbababaaabbaab
        bbaabaabbabbbaaababbbaaabbbbbaabbbababba
        ababababbaaabbabbababbbbbbbbabba
        abbaabbababbbabaabbabbbabbaaaaababbbabababaabbaa
        aabbbabaaaabbbbbababbbbbbabbaaabbabaababaabbabab
        babbbaaababaabbbbbbabbaa
        ababaabaaabbaaababaabbbb
        aabaabaaaabbbbaaaabbabaa
        baaaabbbbbbababbaabaababaabbabaaababbbbabaaabbbbbbbaaabbabbbbbbaaaaaaabaaaaaabbbbbbaaaba
        baaabbabbabbbabaaaabaaba
        baaaabbbbbbbbaababbbbbbb
        baaaabbaaabbbbbbaababaaa
        aabaababbaaaababaabbabba
        aabbbbaaaabbaaaabaababba
        bbaaabaaaaaabbbabbabaabababbabbaaababbababaabbab
        abbabbbaaabaaabbabbabbaa
        aaaaabbbbbbbbbbbabaababbababbbbabbbabbbaabaababb
        bbbbababbbabaabababbbabaaabaabababaaaabbabaabbabbbaabbab
        babbbabbbababbbbabbaabbbabaaaabbbbabbaabaabbabaaabbbbaab
        baabaabbaaababbaaababbbbbbaaabbbabaaaaabbbbbaaabbbbaaaaa
        abaaababaaaabababbaabbab
        abababaaaaaabaaababbabba
        abbbaaabbaabbbbbbaaababb
        baabbbbbbbbbabbbbbaabbba
        abaaabaaabbbbbabbbbabbba
        babbbabbaabaabaaabaaaaaa
        aaabbabbabaaababbbbabbabbabaaaabaabbababbbbbabba
        aabaaaaaababaaaabababaaaabbbbbbababbaabaabbbaaaa
        abaaaabbbbbabaaaaabbbbababaabaaa
        aaabbbabaaaabaaaabbabbab
        bbbbaabbbabbbabbbaaabbbabaabaabbbabbabbbabaaabbaabbababb
        babaabbbbbbbababbbaabaababbaaabb
        abbbbabbbbbbbbbababbaabb
        abaaaabbaabbbbaabaabbbbbbabbabbababbbbbbabbabbaabababaab
        aaabbabbaaaabbbaabbbabaa
        babbbabaabababaaababbaba
        abbaabbababaaaaaaababaaa
        aabbbabbabababbbabbbbbaa
        aababbbbbaaaaabbbaabbaab
        aabbaabaaabaabaaaabbbaab
        abaabbbabababaaaaabababb
        abbbbbbaabaaabaababbabbb
        baabaaaaaaabbabaaabbaabbaabbaaab
        baababbaababbaabbbbabbaa
        ababababbabaaaabbaaabaab
        abbabbbbabbbababaabababb
        bbabbabbbbabaaabaabbbbba
        aaaaaabbbaaaaabbabbababb
        baaabaaababbbabaaababbbbbbaabbab
        bbbbbbbababaababaababbaa
        baaaaaaaabaabbbaababbabbbaaaaaabbaabaaaaababbbbaaabaabaa
        aaabbbabbaaaabbbbabbbabababaaabababaabba
        bbbabbabababbabbbabaaabb
        bbaaabaaaaaaaaaababaaabb
        aabaabbaaabbbaabbaaabaaaababbaab
        abbbbaaabbabbbbbbbbbbbaabbabbabbbabbbbab
        aaababaabaababaabbbaabab
        baabaabbbbabbbabababbaaa
        abbbaaabaabaababbbbbababababbaabbabaaaaaabbbbbaa
        aabbbbbbbbbabbbbabaabaab
        bababaaabbbabbbbababbaba
        bbabaabbbababaaabaaaaabb
        bbbbbbaababaababababbbbb
        bababbbbabbbbbabbaabaaba
        aabaababbbbbbbbabaabbbabbbabbbabbbabbaabaabaaabbababbaba
        ababbbabbaaabbabababbbbbababaabaaabaabaabbbaaaaaaaaabbaabbbababbaaabbabbbbaabbaa
        bbabbbabaaabbabaabaabaaa
        bbabbaabbbaaaaabababbbbb
        bbbbbbaaababbabbaababbaa
        bbbbbbaabbabababaaaabaab
        abbaaabbabbababbabababbaaabbbbababbaabbbabbbbbab
        bbbbababbabbbabbbbaabaaa
        bbaabaabbababbaaaabbaaaabaaaaaba
        abbaabaaababbabbabbbabaaaaaabaab
        baaaababbbabbabbbbbbaabbabbaaaaabaabbbaaaaabbaaaabaaaaab
        bbbbbbbabaabaaabbbabbbbaaabbbbababbaaabb
        bbaaabaaabbaaaaaaaabbaaa
        babaabbbabbabbbbbaaabbbbababaabb
        bbbbabbbbababbbabbabbbaa
        baaaaaababbaaaabbbbaabba
        abbbaaabbbbbababaabbbbbb
        aabbaaaaaababaabbabaabbbbabbaabaaaaaaaaaaababbab
        aaabbabbbbbabababbbbbbab
        babaabaabbbabaaaaaabbababaababba
        abaabbbaabbbbbababaaaaab
        aaabbabbaabababbaabbbbba
        aabaaabaaaaabbbaabbbaabaababbbbbbbbbaabbbbbbaabbbaabbaaa
        abbaabbbbaaaabbbabbbaabb
        bbaaabbaaaababaabbbabaababaaaabaaabbbaabbbaababbbaababba
        babaababbabaaaaaaaaaabaabbbbbaababbabbaa
        bbaaabbabbabbbabaaaaabbb
        aaaabababbababbbaabbbbababbaaaaabaaaaaba
        bbabaabbbbbbbbbaabaaaaab
        bbabaaababaaababbaaaabaa
        aabbaaabaabbbabaabbbabaa
        bababaaabbbbbbbabababaaaabaaaaab
        abaaaabbaaaaababaaaaaaba
        baaaabbaaaababababaabaabbaaaaabbbaababba
        bbababbbbbaaaaabaabababb
        bbbabbbbbbbbaababbbbaabaabbbbbaa
        bbbabbbbabaabaaaababbbbaababaabb
        bbbbababbabbbabbbabbabaa
        baaabababaabbbaabbabaabababbabbaabbaabab
        aaababbabaaaaaaabbbaaaba
        baaaaaabaabaaabaaabaabba
        babbaabababababbaaabbaaa
        bbbababaaaaaaaabbbbaaaab
        bababbaabbbabaabbaaaaaba
        abababbbbbbbabababbabbaa
        aaabbbabbaaabbabaabababa
        aaaabbbababaabbbabbaaaaaabaababbbabbabab
        bbaaaaababaaabaababaaaabbabaabbbaaaabbaa
        abbabaabaabbaaabbababbbabaaabaab
        abbbababaaabbabbaaabbaab
        abbabbbbabaaababaaabbbaa
        aababaababbaabbbaababbaa
        babaabaaaaabaaabbbbbaaab
        aababbbbbaababbbbaabbabb
        bbbbaaaaabaaaaabbbbaabaabbababbabaaabbaabbbaabab
        ababababbaaaabbbbbbaabbbabbbbbaa
        bbabbbbbbbababbbbabbabab
        bbbbbbbbbbabbaabbabbbaaaaabbbabaaabbbbba
        bbabbbabaabbaaabbbbabbba
        aabaabbbbbaababbaabababa
        bbaabaabbbbbababbbbbbaaa
        bababbaaaaaaaabbabbbbaaabbbabaababbbbababbabbbaaabbbaabb
        aabbbbbbbbbbabaabbbbbaaa
        ababbbaabbbaabbbbaaaaaabababaaababbbababbbabababbbaabbbb
        aabbbbbbbaabbbabbbabbaaa
        bababbababaaaabbabaabbab
        aababbbbababababaaabbaaa
        bbbbababbbabbbabbaababaa
        abbbbaaabbaaabbbbaaaaabbaaaaabba
        abbaabbabbaabaabbabbaabb
        baaaabbbaaabbbbabaaabaab
        baaabaaabbaababaaaabbaab
        baabbbabbbbbbaabbabaabbbbaababab
        bbabababaaabbbababaababa
        aabaabaabbabbbbababbaaababbbabbbaabbabaabbbbabbabbbbbbbb
        abaaaababbbbababaabababb
        abbabbbbaaaabbbbabbabbbbababaabb
        aababaabaaaaabaababaaaba
        abbbbaaabbaaaaabaaabbaaa
        aabaaabaaabbaaaaaaaabaab
        bbaaabbbbbbababaaabababa
        bbbabbabbaaaababbabaabbbaabbbbbbabbaaaaabbababba
        bbbbababbaaaaabbabbbbbabbaabbabb
        bbabaabbbbbbaabbbaabbbabaabaaaaabaabbaaa
        abbaaaabbbbbabbbabaabaaa
        aaabbbbaaaababbabaaaaaba
        bababbaabaaaabbaaaabbaaa
        ababababaaaabaaaababbaab
        aabaaababbabaabaabaabbbb
        abbaabbbbbbbabbbbbbbbaabbbababbbaaabbaaabbababaa
        abbabbbbaaaabaaababbbabaabbbabaaaabbabaa
        aababbbbbaabaabbbbbabaabaababbbbbbbbaaabbbabbaaa
        babaaaabbbabbaabbaababba
        bbabbbabbbbbabaabaaaabaa
        aaaaaabbaabbbababaaabaab
        abaaaababbbbbbaaabaabaab
        abaabbbabaaaaabaaaaabbaaaaaababbabbbaabb
        bbbbaaaaabbaabbabbbbabbbbbabaabaaaabaaab
        bbabaaabaababbbabbababbbababbaaa
        bbbbbbbbbbabaaabbaabbabb
        bbbabbbbabbaabbbbaababbbabbababb
        bbaaabbabaabbbaabaababab
        bbabaabaabbbababbaaabbabbbbbaaab
        abbbbaaaababababbabaabbbabbbbbbb
        babaaaaaaabaaabababaababbabbaabbbbaabaaa
        baaaabbabbaababaabbababb
        aaaaaaabbaaaabbbbbbaaaaa
        aabaaaaabbbabaabbaababaa
        abbabaabbbabbbbbbaabaaaaaaaabbaaaaabbaab
        baaaabababbbbaaabaababaa
        babaaaaabaaabbbbaaabaaaa
        aaabbababaaaaaaaabbbabaa
        bbabaaabbaaaababaaabaaaa
        bbaaabbbbabbbbbaaabbbbbaaabbbbbabbabaabababbbbbaabbabbaabbbabbbaaaaaaaababbbbababaabaabb
        aabaaaaaaabababababbabaaaaabbaaa
        bbbabaaabbaaaaaabbbaaaab
        abaaaaaabbabaaaabbbaabbabbababaa
        bbaaabbaaabbbabaababbbbb
        bbabaaabbababbbababaaaabbabaaaba
        abbabbbabaaabbbaaabaaaab
        baaabbbbbabbbaaabbbaaabb
        bbaaabbaababbbababaaabba
        aaaaabaaaabbbbabaabbbbaaaababbbbbabaabbbbbabbbaa
        bbaaaaaaabababbbbabbbbaa
        ababbabbaabaaaaaaaababbb
        babbabbabaabaaabbabaababbbbaabbbbbababababaababb
        abbaaaaabababbbbaaaabababbaaaaabaabaabbb
        bbaaabbbbaaaabbbaaabbaaa
        bbabaaabbaaaaaaabbabbaba
        bbabaabaaabaaabbbaaabababaaaabaa
        bbbbababbbbabaaababbbaaaababababbaabbbaaabaabbaaaaabbbbb
        bbaaaaabbbaaabaabaababab
        aaaabaaabaaabbbaaababbab
        aabbaababbabbbbbabbababa
        baaaaaaaaabbaabaaabbbbabbaaaaaababaababb
        aabbaaabababababaaaaabbb
        aabaaaaabbbbaabaaaaabaab
        bbaaaaabbbbabaaaabbbabba
        babaaaababbabbbbbbbaaaaa
        bbbbaabaaaabbbbababbaabaabbbababaabaaaabbaababba
        bababbbbbabbbaaabbabbaba
        abbaaaabbabbbaaabaabbabb
        aabbbababaaaababbabbbbab
        abaaabaabbbbabababbbaaababbabaaababaabba
        bbbabbabaaabbabbabbabbbabaaababbaaaababababbbbabbabbabaaabaaabaaaaaabaaababbaabb
        abbbaaabbabaaaaabababbbabababbbbabaabbaa
        aabaaaaabaababbbbaaaaaba
        aaaaaaabbabaaaababbbbaab
        aabaaababbaaabbabaaabaaaaabbaaaababaababbbbbbbbbabaaaaab
        bbbbbbbbbbaabababbbbbbbbaabbabbb
        abbbbbbaaabbbbbbaaaaabaabbbbababbabbabab
        baaababaabbbaaabababbbbb
        bababbbabbabaaabbbabababbbbbabbaabbbabbb
        baaabbbabaabbabbbbabbaba
        ababbbbabbaaaabbbaababbbaaabbbbbbbabbbabaabaabaabbaabbaabbbaabbaaabaabba
        bababbbaababbabbaabaabaabbbbaaab
        aaabaabaaabaabbabbaabbaaabbbaaab
        abaaababbbbbbaabbaabbaab
        aaabbabbaaaaaabbaaabaabb
        bbbabbabbbbabaabbbabbbabbabbaabbabbaaabb
        bbaababaabbaabaaaabbbbba
        ababaaaabaabbbaaaaaababb
        babaabababaaabaabbbabbbbbbbbbbbbbabaabbbabbbbbaaabbaababbaaabaababbbbaba
        ababbabbbaababababbababbbbabaabaababbabbaabbbbaa
        baaaabababbaababaaabaaaaaaabbbaabbababbabaaaaaabbaaabbaaaabaaaaababbbbababbabaab
        aaabbbbaabbabbbbbababaaabababbbbabaabaabaabbbaaaabbbbaab
        aabbbbbbabbaaaabaaabaabb
        bbbaabbbaaaaaaabbbaabaabbababbbaabababbaabbaaabb
        bbbbaaaababbbabababaaabb
        bbaaabbbbababbbababbbaab
        bbabbabbaabbbbabaaaabababaababaa
        abaaabaaabaaababbaababaa
        aabaaabbbaabaaaabbbbbbab
        baabaaabbabaabababbbabaa
        ababbabbbaabbbbbbabbabaa
        abbbbaaabbbabbbbbababaaa
        bbbbbbaabababaaaabaababa
        aabbaabaabaaaabbbaabbaba
        bbbbbbaaaabbaaaaaabbaabaaaaaabbb
        aaaabbbbaaaaabaabaaaaaabaaabaabb
        abaaababbaaaababaaaaaaaaabaabbabababbaab
        babababbabbabaabaabaabbb
        babaaaabbbabbabbbaababab
        aaabbababbbbbbbaabbbbabbbbbbaabbaaabbabbaaaabababaaabbaaababbaab
        abaabbbabaaaabbbabbbababbaaaabbbbabbabaa
        aabbbbaababbaababbbaabaa
        bbaabababbaababaaaabbbaa
        babaababbbbbabaaaabaaabaabbabbbbaaaaaabbbbaababbabaabaaaaabbbaaababbbbbb
        baababbbbaaaaaaabbabbaab
        aaaabbbaababababbbbbbabb
        bbaaaaaaababbbaaaabbabbabbabaaababbbbbbabbbaaaaabbaababbbaababaa
        aaaaaaabbbaaabaabbbabaabbbabaababbaaabaabbaababb
        babaaaabaabbbbaabaaaaaabbaaaaaba
        aabbbaabababbbabbabaaabababbbbbabaababbaaabbaabaaabbaaaabaabbbbbbaaaababbbaabaab
        babaaaaaababbbabaabababb
        babbabbbabbaababbabbbaab
        baabbbabbabbbabbabbbbaaaaaaaaaaaaabaabbaaaababbbabbbaabb
        bbbaabbbbababbabbbbaaabb
        babaabbbbaabaaabaaabababbbaabbbbbbbabbba
        bababbabbaaabbbabbbbbabb
        bbabbbabbbabbbbbbbbaaaab
        baabaaaaabbaaaabababaaaaaaabbaaa
        ababbbaabbabbabbbaabbbba
        abbaabbbaaaabbbbaaaaabba
        bbaababbbbababaaabbaababbabbabaababbbbab
        bbbabbbbaaaaaabbbbaabaaababaaabbaaaababb
        baaaaaabbbbbbabbbbaabbaaababbbbb
        abbbbbbbababbaaaababaabababbabbaabaaabaababaaaaaabababab
        ababababbbaaaaaabababbbbabaaabbb
        baaabbabbbabbbbbbbaaaaabaabababa
        baaababaabbaaaaaabababaababbaababaaaabbbbbbbbaaaabaabbaa
        abbbbbbaababbaabbababaaaaabbbbbaaababbbbaaabbbbb
        aaababaaabaaabaaabaaabbb
        baabaaaabbbbbbaabbaabbba
        ababbbabbbbbabbababbabab
        bbbababaababbbaaaaaaaaaaaaaabababaababab
        aababbbbbabababbbbaaaabb
        bbaaaaaabbabaabaababbbba
        bbabaabaabbabaabbaabbabb
        abbbabaabbbbbbabaaaaabaababaabab
        baabbbaabbbababaaaaabbab
        babaabababbaabbbabaabbbb
        aabaaaaaababbabbabaabbbb
    "#};
}
