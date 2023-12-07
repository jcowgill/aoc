use itertools::Itertools;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Hand(u8, [u8; 5]);

fn hand_type(hand: [u8; 5]) -> u8 {
    let mut counts = [0u8; 15];
    for c in hand {
        counts[c as usize] += 1;
    }

    let jokers = counts[1];
    counts[1] = 0;

    let max_pos = counts.iter().position_max().unwrap();
    let max = counts[max_pos];
    counts[max_pos] = 0;

    match max + jokers {
        0 | 1 => 0,
        2 => 1 + u8::from(counts.contains(&2)),
        3 => 3 + u8::from(counts.contains(&2)),
        m => m + 1,
    }
}

impl Hand {
    fn from_cards(cards: &str, j_is_joker: bool) -> Hand {
        Self::from_card_ranks(
            cards
                .chars()
                .map(|c| match c {
                    '2'..='9' => (c as u8) - b'0',
                    'T' => 10,
                    'J' if j_is_joker => 1,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => panic!("invalid card"),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    fn from_card_ranks(cards: [u8; 5]) -> Hand {
        Hand(hand_type(cards), cards)
    }
}

fn solve(input: &str, j_is_joker: bool) -> String {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_ascii_whitespace().collect_tuple().unwrap();
            (
                Hand::from_cards(hand, j_is_joker),
                bid.parse::<u32>().unwrap(),
            )
        })
        .sorted_unstable()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

pub fn star1(input: &str) -> String {
    solve(input, false)
}

pub fn star2(input: &str) -> String {
    solve(input, true)
}
