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
