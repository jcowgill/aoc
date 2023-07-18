pub fn star1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            let mut ans: Vec<char> = group.chars().filter(char::is_ascii_alphabetic).collect();
            ans.sort_unstable();
            ans.dedup();
            ans.len()
        })
        .sum::<usize>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    input
        .split("\n\n")
        .map(|group| {
            let mut lines = group.lines();
            let mut all: Vec<char> = lines.next().unwrap().chars().collect();
            for l in lines {
                all.retain(|&c| l.chars().any(|c2| c == c2))
            }
            all.len()
        })
        .sum::<usize>()
        .to_string()
}
