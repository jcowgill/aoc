fn simulate_fish(input: &str, days: u32) -> String {
    let mut counts = [0; 9];

    for fish in input.split(',') {
        counts[fish.trim().parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..days {
        counts = [
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[7] + counts[0],
            counts[8],
            counts[0],
        ];
    }

    counts.iter().sum::<u64>().to_string()
}

pub fn star1(input: &str) -> String {
    simulate_fish(input, 80)
}

pub fn star2(input: &str) -> String {
    simulate_fish(input, 256)
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, "3,4,3,1,2", "5934");
    star_test!(me1, star1, ME, "371379");

    star_test!(example1b, star2, "3,4,3,1,2", "26984457539");
    star_test!(me2, star2, ME, "1674303997472");

    const ME: &str = indoc! {"
        3,4,3,1,2,1,5,1,1,1,1,4,1,2,1,1,2,1,1,1,3,4,4,4,1,3,2,1,3,4,1,1,3,4,2,5,5,3,3,3,5,1,4,1,2,3,1,1,1,4,1,4,1,5,3,3,1,4,1,5,1,2,2,1,1,5,5,2,5,1,1,1,1,3,1,4,1,1,1,4,1,1,1,5,2,3,5,3,4,1,1,1,1,1,2,2,1,1,1,1,1,1,5,5,1,3,3,1,2,1,3,1,5,1,1,4,1,1,2,4,1,5,1,1,3,3,3,4,2,4,1,1,5,1,1,1,1,4,4,1,1,1,3,1,1,2,1,3,1,1,1,1,5,3,3,2,2,1,4,3,3,2,1,3,3,1,2,5,1,3,5,2,2,1,1,1,1,5,1,2,1,1,3,5,4,2,3,1,1,1,4,1,3,2,1,5,4,5,1,4,5,1,3,3,5,1,2,1,1,3,3,1,5,3,1,1,1,3,2,5,5,1,1,4,2,1,2,1,1,5,5,1,4,1,1,3,1,5,2,5,3,1,5,2,2,1,1,5,1,5,1,2,1,3,1,1,1,2,3,2,1,4,1,1,1,1,5,4,1,4,5,1,4,3,4,1,1,1,1,2,5,4,1,1,3,1,2,1,1,2,1,1,1,2,1,1,1,1,1,4
    "};
}
