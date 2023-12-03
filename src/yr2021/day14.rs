use std::collections::HashMap;

use itertools::Itertools;

type Pair = (u8, u8);

fn parse_input(input: &str) -> (Vec<u8>, HashMap<Pair, u8>) {
    let mut lines = input.lines();
    let start_chars = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c as u8)
        .collect();
    let mappings = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let chars: Vec<u8> = l
                .chars()
                .filter(char::is_ascii_alphabetic)
                .map(|c| c as u8)
                .collect();
            ((chars[0], chars[1]), chars[2])
        })
        .collect();

    (start_chars, mappings)
}

fn star_common(input: &str, depth: usize) -> String {
    let (start_chars, mappings) = parse_input(input);
    let mut char_freqs = start_chars.iter().copied().counts();
    let mut pair_freqs = start_chars.into_iter().tuple_windows().counts();

    for _ in 0..depth {
        for (pair, freq) in pair_freqs
            .iter()
            .map(|(&pair, &freq)| (pair, freq))
            .collect_vec()
        {
            let middle = mappings[&pair];
            *pair_freqs.get_mut(&pair).unwrap() -= freq;
            *pair_freqs.entry((pair.0, middle)).or_default() += freq;
            *pair_freqs.entry((middle, pair.1)).or_default() += freq;
            *char_freqs.entry(middle).or_default() += freq;
        }
    }

    let (min, max) = char_freqs.into_values().minmax().into_option().unwrap();
    (max - min).to_string()
}

pub fn star1(input: &str) -> String {
    star_common(input, 10)
}

pub fn star2(input: &str) -> String {
    star_common(input, 40)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "1588");
    star_test!(me1, star1, ME, "3058");

    star_test!(example1b, star2, IN1, "2188189693529");
    star_test!(me2, star2, ME, "3447389044530");

    const IN1: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    const ME: &str = indoc! {"
        SVCHKVFKCSHVFNBKKPOC

        NC -> H
        PK -> V
        SO -> C
        PH -> F
        FP -> N
        PN -> B
        NP -> V
        NK -> S
        FV -> P
        SB -> S
        VN -> F
        SC -> H
        OB -> F
        ON -> O
        HN -> V
        HC -> F
        SN -> K
        CB -> H
        OP -> K
        HP -> H
        KS -> S
        BC -> S
        VB -> V
        FC -> B
        BH -> C
        HH -> O
        KH -> S
        VF -> F
        PF -> P
        VV -> F
        PP -> V
        BO -> H
        BF -> B
        PS -> K
        FO -> O
        KF -> O
        FN -> H
        CK -> B
        VP -> V
        HK -> F
        OV -> P
        CS -> V
        FF -> P
        OH -> N
        VS -> H
        VO -> O
        CP -> O
        KC -> V
        KV -> P
        BK -> B
        VK -> S
        NF -> V
        OO -> V
        FH -> H
        CN -> O
        SP -> B
        KN -> V
        OF -> H
        NV -> H
        FK -> B
        PV -> N
        NB -> B
        KK -> P
        VH -> P
        CC -> B
        HV -> V
        OC -> H
        PO -> V
        NO -> O
        BP -> C
        NH -> H
        BN -> O
        BV -> S
        CV -> B
        HS -> O
        NN -> S
        NS -> P
        KB -> F
        CO -> H
        HO -> P
        PB -> B
        BS -> P
        SH -> H
        FS -> V
        SF -> O
        OK -> F
        KP -> S
        BB -> C
        PC -> B
        OS -> C
        SV -> N
        SK -> K
        KO -> C
        SS -> V
        CF -> C
        HB -> K
        VC -> B
        CH -> P
        HF -> K
        FB -> V
    "};
}
