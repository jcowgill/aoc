/// Sum of all digits that match the next digit in the list
pub fn star1(input: &str) -> String {
    if input.is_empty() {
        return String::from("0");
    }

    let chars: Vec<char> = input.chars().collect();
    let mut last_char = *chars.last().unwrap();
    let mut sum = 0;

    for c in chars {
        if c == last_char {
            sum += c.to_digit(10).unwrap();
        }

        last_char = c;
    }

    sum.to_string()
}

/// Sum of all digits that match the digit halfway around the list
pub fn star2(input: &str) -> String {
    // Get a duplicated character list (string appended to itself)
    let mut chars: Vec<char> = input.chars().collect();
    let mut chars_cloned = chars.clone();
    chars.append(&mut chars_cloned);

    let mut sum = 0;
    for (i, c) in chars.iter().enumerate() {
        // Exit if we get to the end of the string
        if i >= chars.len() / 2 {
            break;
        }

        // Add to sum if the current character matches the one halfway around
        if *c == chars[i + chars.len() / 4] {
            sum += (*c).to_digit(10).unwrap();
        }
    }

    sum.to_string()
}
