use std::collections::HashSet;

/// Parses the input string and returns an iterator over each integer
fn parse_input(input: &str) -> impl Iterator<Item = i32> + Clone + '_ {
    input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
}

/// Calculate resulting frequency from given list of signed integers
pub fn star1(input: &str) -> String {
    parse_input(input).sum::<i32>().to_string()
}

/// Find the first intermediate frequency which appears twice
pub fn star2(input: &str) -> String {
    let mut set = HashSet::new();
    let mut total: i32 = 0;

    // Add initial zero to the set
    set.insert(0);

    // For each integer, add once to the set or return if it's already there
    for integer in parse_input(input).cycle() {
        total += integer;
        if !set.insert(total) {
            return total.to_string();
        }
    }

    unreachable!()
}
