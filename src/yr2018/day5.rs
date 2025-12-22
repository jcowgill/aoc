/// Predicate which returns true if two units react
fn units_react(a: char, b: char) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

/// React a polymer returning a new vector
fn react<I: Iterator<Item = char>>(polymer: I) -> Vec<char> {
    let mut result = Vec::new();

    for c in polymer {
        assert!(c.is_ascii_alphabetic());

        // React with most recent character, or append to the result
        if !result.is_empty() && units_react(c, result[result.len() - 1]) {
            result.pop();
        } else {
            result.push(c);
        }
    }

    result
}

pub fn star1(input: &str) -> String {
    react(input.chars()).len().to_string()
}

pub fn star2(input: &str) -> String {
    (b'a'..=b'z')
        .map(char::from)
        .map(|r| react(input.chars().filter(|c| c.to_ascii_lowercase() != r)).len())
        .min()
        .unwrap()
        .to_string()
}
