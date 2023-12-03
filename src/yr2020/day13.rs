pub fn star1(input: &str) -> String {
    let (target_str, buses_str) = input.split_once('\n').unwrap();
    let target = target_str.parse::<u32>().unwrap();
    let (wait, bus) = buses_str
        .split(',')
        .flat_map(|s| s.parse::<u32>().ok())
        .map(|b| match target % b {
            0 => (0, b),
            r => (b - r, b),
        })
        .min()
        .unwrap();
    (wait * bus).to_string()
}

pub fn star2(input: &str) -> String {
    let constraints: Vec<(u64, u64)> = input
        .split_once('\n')
        .unwrap()
        .1
        .split(',')
        .enumerate()
        .flat_map(|(i, s)| {
            s.parse()
                .ok()
                .map(|b| (b, (-(i as i32)).rem_euclid(b as i32) as u64))
        })
        .collect();

    let (mut n1, mut x) = constraints[0];
    for (n2, a) in constraints.into_iter().skip(1) {
        // Given:
        //  x % n1 == 0
        // Find minimum new_x such that:
        //  new_x % n1 == 0
        //  new_x % n2 == a
        for i in 0.. {
            let new_x = x + i * n1;
            if new_x % n2 == a {
                x = new_x;
                n1 *= n2;
                break;
            }
        }
    }

    x.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "295");
    star_test!(me1, star1, ME, "171");

    star_test!(example1b, star2, IN1, "1068781");
    star_test!(example2, star2, IN2, "3417");
    star_test!(example3, star2, IN3, "754018");
    star_test!(example4, star2, IN4, "779210");
    star_test!(example5, star2, IN5, "1261476");
    star_test!(example6, star2, IN6, "1202161486");
    star_test!(me2, star2, ME, "539746751134958");

    const IN1: &str = indoc! {"
        939
        7,13,x,x,59,x,31,19
    "};

    const IN2: &str = indoc! {"
        0
        17,x,13,19
    "};

    const IN3: &str = indoc! {"
        0
        67,7,59,61
    "};

    const IN4: &str = indoc! {"
        0
        67,x,7,59,61
    "};

    const IN5: &str = indoc! {"
        0
        67,7,x,59,61
    "};

    const IN6: &str = indoc! {"
        0
        1789,37,47,1889
    "};

    const ME: &str = indoc! {"
        1000417
        23,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,479,x,x,x,x,x,x,x,x,x,x,x,x,13,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,373,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19
    "};
}
