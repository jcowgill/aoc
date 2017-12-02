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

pub fn star2(input: &str) -> String {
    return String::from("Hello Worldz");
}
