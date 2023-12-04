use std::collections::HashSet;

fn parse_nums(nums: &str) -> HashSet<u32> {
    nums.split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn winning_cards(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| {
        let (l_str, r_str) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        parse_nums(l_str).intersection(&parse_nums(r_str)).count()
    })
}

pub fn star1(input: &str) -> String {
    winning_cards(input)
        .map(|w| if w > 0 { 1 << (w - 1) } else { 0 })
        .sum::<usize>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let winnings: Vec<_> = winning_cards(input).collect();
    let mut cards = vec![1; winnings.len()];

    for i in 0..cards.len() {
        let card_num = cards[i];
        for j in 0..winnings[i] {
            if let Some(n) = cards.get_mut(i + j + 1) {
                *n += card_num;
            }
        }
    }

    cards.into_iter().sum::<u32>().to_string()
}
