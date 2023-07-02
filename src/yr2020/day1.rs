use std::collections::HashSet;

fn parse_input(input: &str) -> HashSet<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn star1(input: &str) -> String {
    let nums = parse_input(input);
    let a = *nums.iter().find(|&&n| nums.contains(&(2020 - n))).unwrap();
    (a * (2020 - a)).to_string()
}

pub fn star2(input: &str) -> String {
    let nums = parse_input(input);
    for &a in nums.iter() {
        if let Some(&b) = nums.iter().find(|&&n| nums.contains(&(2020 - a - n))) {
            return (a * b * (2020 - a - b)).to_string();
        }
    }

    panic!("invalid input")
}
