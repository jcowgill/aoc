fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.split(',').flat_map(|r| {
        let (a, b) = r.split_once('-').expect("invalid input");
        let an = a.parse().expect("invalid number");
        let bn = b.parse().expect("invalid number");
        an..=bn
    })
}

fn get_divisor(digits: u32, repeats: u32) -> u64 {
    assert_ne!(digits, 0);
    assert_ne!(digits, 0);
    assert!(repeats <= digits);
    assert_eq!(digits % repeats, 0);

    let mut result = 0;
    let mut i = 0;
    while i < digits {
        result += 10u64.pow(i);
        i += digits / repeats;
    }
    result
}

fn is_invalid1(value: u64) -> bool {
    if value > 0 {
        let digits = value.ilog10() + 1;
        digits.is_multiple_of(2) && value.is_multiple_of(10u64.pow(digits / 2) + 1)
    } else {
        false
    }
}

fn is_invalid2(value: u64) -> bool {
    if value > 0 {
        let digits = value.ilog10() + 1;
        (2..=digits)
            .any(|d| digits.is_multiple_of(d) && value.is_multiple_of(get_divisor(digits, d)))
    } else {
        false
    }
}

pub fn star1(input: &str) -> String {
    parse(input)
        .filter(|&v| is_invalid1(v))
        .sum::<u64>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    parse(input)
        .filter(|&v| is_invalid2(v))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn divisor() {
        assert_eq!(get_divisor(2, 2), 11);
        assert_eq!(get_divisor(4, 2), 101);
        assert_eq!(get_divisor(6, 3), 10101);
    }
}
