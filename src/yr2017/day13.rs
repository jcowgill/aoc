/// Parse the list layer depths into a vector
fn parse_depths(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split(':').map(|s| s.trim().parse().unwrap()).collect();
            assert_eq!(parts.len(), 2);
            (parts[0], parts[1])
        })
        .collect()
}

/// Calculates the severity of performing a walk
///  time_offset = time to start the walk at
///  depths      = the depths of each layer
/// Returns Some(severity) or None if not caught
fn walk_severity(time_offset: i32, depths: &[(i32, i32)]) -> Option<i32> {
    depths.iter().fold(None, |sum, &(layer, depth)| {
        if (time_offset + layer) % (2 * depth - 2) == 0 {
            Some(sum.unwrap_or(0) + layer * depth)
        } else {
            sum
        }
    })
}

/// Find severity of whole firewall trip
pub fn star1(input: &str) -> String {
    walk_severity(0, &parse_depths(input))
        .unwrap_or(0)
        .to_string()
}

/// Find smallest time delay which gets through safely
pub fn star2(input: &str) -> String {
    let depths = parse_depths(input);
    let mut time_offset = 0;

    while walk_severity(time_offset, &depths).is_some() {
        time_offset += 1;
    }

    time_offset.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "24");
    star_test!(me1, star1, ME, "1300");

    star_test!(example1b, star2, IN1, "10");
    star_test!(me2, star2, ME, "3870382");

    const IN1: &str = indoc! {"
        0: 3
        1: 2
        4: 4
        6: 4
    "};

    const ME: &str = indoc! {"
        0: 3
        1: 2
        2: 6
        4: 4
        6: 4
        8: 8
        10: 6
        12: 8
        14: 5
        16: 6
        18: 8
        20: 8
        22: 12
        24: 6
        26: 9
        28: 8
        30: 12
        32: 12
        34: 17
        36: 12
        38: 8
        40: 12
        42: 12
        44: 10
        46: 12
        48: 12
        50: 12
        52: 14
        54: 14
        56: 10
        58: 14
        60: 12
        62: 14
        64: 14
        66: 14
        68: 14
        70: 14
        72: 14
        74: 14
        76: 14
        86: 14
        94: 20
        96: 18
    "};
}
