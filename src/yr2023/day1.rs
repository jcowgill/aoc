const WORDS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn star1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut digits = line.bytes().filter(u8::is_ascii_digit);
            let d1 = digits.next().unwrap();
            let d2 = digits.last().unwrap_or(d1);
            u32::from((d1 - b'0') * 10 + (d2 - b'0'))
        })
        .sum::<u32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut first_pos = bytes.iter().position(u8::is_ascii_digit);
            let mut first = first_pos.map(|p| bytes[p] - b'0');

            for (i, word) in WORDS.iter().enumerate() {
                let maybe_pos = bytes
                    .windows(word.len())
                    .take(first_pos.unwrap_or(usize::MAX))
                    .position(|w| w == *word);
                if let Some(pos) = maybe_pos {
                    first_pos = Some(pos);
                    first = Some(i as u8 + 1);
                }
            }

            let mut last_pos = bytes.iter().rposition(u8::is_ascii_digit);
            let mut last = last_pos.map(|p| bytes[p] - b'0');

            for (i, word) in WORDS.iter().enumerate() {
                let maybe_pos = bytes
                    .windows(word.len())
                    .skip(last_pos.unwrap_or(0))
                    .rposition(|w| w == *word);
                if let Some(pos) = maybe_pos {
                    *last_pos.get_or_insert(0) += pos;
                    last = Some(i as u8 + 1);
                }
            }

            u32::from(first.unwrap() * 10 + last.unwrap())
        })
        .sum::<u32>()
        .to_string()
}
