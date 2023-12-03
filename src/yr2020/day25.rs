const MODULUS: u64 = 20_201_227;

fn pow_modulo(base: u64, exponent: u64) -> u64 {
    (0..exponent).fold(1, |acc, _| (acc * base) % MODULUS)
}

fn discrete_log(base: u64, result: u64) -> u64 {
    let mut prev = 1;
    for n in 0..MODULUS {
        if prev == result {
            return n;
        }
        prev = (prev * base) % MODULUS;
    }
    panic!("no solution");
}

pub fn star1(input: &str) -> String {
    let keys: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();
    pow_modulo(keys[0], discrete_log(7, keys[1])).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1, star1, IN1, "14897079");
    star_test!(me, star1, ME, "11328376");

    const IN1: &str = indoc! {"
        5764801
        17807724
    "};

    const ME: &str = indoc! {"
        10705932
        12301431
    "};
}
