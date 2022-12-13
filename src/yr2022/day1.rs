fn add_value<const N: usize>(max: &mut [u32; N], value: u32) {
    let min = max.iter_mut().min().unwrap();
    if *min < value {
        *min = value;
    }
}

fn top_k<const N: usize>(input: &str) -> String {
    let mut max = [0; N];
    let mut current = 0;

    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            add_value(&mut max, current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    add_value(&mut max, current);
    max.into_iter().sum::<u32>().to_string()
}

pub fn star1(input: &str) -> String {
    top_k::<1>(input)
}

pub fn star2(input: &str) -> String {
    top_k::<3>(input)
}
