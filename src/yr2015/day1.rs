/// Appartment building floor using ( and ) characters
pub fn star1(input: &str) -> String {
    let up = input.chars().filter(|&c| c == '(').count() as i32;
    let down = input.chars().filter(|&c| c == ')').count() as i32;
    (up - down).to_string()
}

/// Appartment building, when do you enter basement?
pub fn star2(input: &str) -> String {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1
        };
        if c == ')' {
            floor -= 1
        };
        if floor < 0 {
            return (i + 1).to_string();
        }
    }

    panic!("never enters basement!")
}
