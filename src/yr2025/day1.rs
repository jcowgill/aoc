fn parse(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let c1 = chars.next().expect("invald direction");
            let n: i32 = chars.as_str().parse().expect("invalid number");
            match c1 {
                'R' => n,
                'L' => -n,
                _ => panic!("invalid direction"),
            }
        })
        .filter(|&i| i != 0)
}

pub fn star1(input: &str) -> String {
    let mut zeros = 0;
    let mut acc = 50;

    for v in parse(input) {
        acc = (acc + v).rem_euclid(100);
        if acc == 0 {
            zeros += 1;
        }
    }

    zeros.to_string()
}

pub fn star2(input: &str) -> String {
    let mut zeros = 0;
    let mut acc = 50;

    for v in parse(input) {
        let next = acc + v;

        if next == 0 {
            zeros += 1;
        } else if next > 0 {
            zeros += next.div_euclid(100);
        } else if acc == 0 {
            zeros -= next / 100;
        } else {
            zeros += 1 - next / 100;
        }

        acc = next.rem_euclid(100);
    }

    zeros.to_string()
}
