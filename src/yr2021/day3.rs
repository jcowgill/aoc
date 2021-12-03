/// Interprets an iterator as a string of binary digits and converts it into a u32
fn map_fold_binary_number<T, P: FnMut(T) -> bool>(
    iter: impl Iterator<Item = T>,
    mut pred: P,
) -> u32 {
    iter.map(|d| if pred(d) { 1 } else { 0 })
        .fold(0, |acc, v| (acc << 1) + v)
}

pub fn star1(input: &str) -> String {
    let lines: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    let line_count = lines.len();
    let digit_count = lines[0].len();
    let mut digit_sums = vec![0; digit_count];

    for line in lines {
        for (i, &b) in line.iter().enumerate() {
            if b == b'1' {
                digit_sums[i] += 1;
            }
        }
    }

    let gamma = map_fold_binary_number(digit_sums.iter(), |d| d * 2 >= line_count);
    let epsilon = !gamma & ((1 << digit_count) - 1);

    (gamma * epsilon).to_string()
}

fn find_rating<'a>(
    digits: &[&'a [u8]],
    start_digit: usize,
    one_value: u8,
    swap_order: bool,
) -> &'a [u8] {
    assert!(!digits.is_empty());

    if digits.len() == 1 {
        digits[0]
    } else {
        let middle = digits.partition_point(|line| line[start_digit] != one_value);
        if (middle * 2 > digits.len()) ^ swap_order {
            find_rating(&digits[..middle], start_digit + 1, one_value, swap_order)
        } else {
            find_rating(&digits[middle..], start_digit + 1, one_value, swap_order)
        }
    }
}

pub fn star2(input: &str) -> String {
    let mut lines: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
    lines.sort_unstable();

    let oxygen_rating =
        map_fold_binary_number(find_rating(&lines, 0, b'1', false).iter(), |&b| b == b'1');
    let co2_rating =
        map_fold_binary_number(find_rating(&lines, 0, b'1', true).iter(), |&b| b == b'1');

    (oxygen_rating * co2_rating).to_string()
}
