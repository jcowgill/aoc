fn parse_input(input: &str) -> Vec<u32> {
    let mut nums: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums.last().unwrap() + 3);
    nums
}

pub fn star1(input: &str) -> String {
    let nums = parse_input(input);
    let mut jumps = [0, 0, 0];
    for w in nums.windows(2) {
        jumps[(w[1] - w[0] - 1) as usize] += 1;
    }

    (jumps[0] * jumps[2]).to_string()
}

pub fn star2(input: &str) -> String {
    let nums = parse_input(input);
    let mut arrangements = vec![1u64; nums.len()];

    for (i, n1) in nums[..nums.len() - 2].iter().enumerate().rev() {
        arrangements[i] = arrangements[i + 1];
        if nums[i + 2] - n1 <= 3 {
            arrangements[i] += arrangements[i + 2];
        }
        if i + 3 < nums.len() && nums[i + 3] - n1 <= 3 {
            arrangements[i] += arrangements[i + 3];
        }
    }

    arrangements[0].to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "35");
    star_test!(example2a, star1, IN2, "220");
    star_test!(me1, star1, ME, "2112");

    star_test!(example1b, star2, IN1, "8");
    star_test!(example2b, star2, IN2, "19208");
    star_test!(me2, star2, ME, "3022415986688");

    const IN1: &str = indoc! {"
        16
        10
        15
        5
        1
        11
        7
        19
        6
        12
        4
    "};

    const IN2: &str = indoc! {"
        28
        33
        18
        42
        31
        14
        46
        20
        48
        47
        24
        23
        49
        45
        19
        38
        39
        11
        1
        32
        25
        35
        8
        17
        7
        9
        4
        2
        34
        10
        3
    "};

    const ME: &str = indoc! {"
        8
        131
        91
        35
        47
        116
        105
        121
        56
        62
        94
        72
        13
        82
        156
        102
        12
        59
        31
        138
        46
        120
        7
        127
        126
        111
        2
        123
        22
        69
        18
        157
        75
        149
        88
        81
        23
        98
        132
        1
        63
        142
        37
        133
        61
        112
        122
        128
        155
        145
        139
        66
        42
        134
        24
        60
        9
        28
        17
        29
        101
        148
        96
        68
        25
        19
        6
        67
        113
        55
        40
        135
        97
        79
        48
        159
        14
        43
        86
        36
        41
        85
        87
        119
        30
        108
        80
        152
        158
        151
        32
        78
        150
        95
        3
        52
        49
    "};
}
