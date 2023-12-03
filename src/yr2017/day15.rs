/// Number of iterations used in star 1
const STAR1_ITERATIONS: usize = 40000000;

/// Number of iterations used in star 2
const STAR2_ITERATIONS: usize = 5000000;

/// Multiplication factor used by generator A
const FACTOR_A: u16 = 16807;

/// Multiplication factor used by generator B
const FACTOR_B: u16 = 48271;

/// Parses the two initial values to give the generators
fn parse_values(input: &str) -> Vec<u32> {
    let values: Vec<u32> = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
        .collect();

    assert_eq!(values.len(), 2);
    values
}

/// Count the number of good judgements by running A and B over a number of iterations
fn count_judgement<A, B>(mut a: A, mut b: B, iterations: usize) -> usize
where
    A: Iterator<Item = u16>,
    B: Iterator<Item = u16>,
{
    (0..iterations)
        .filter(|_| a.next().unwrap() == b.next().unwrap())
        .count()
}

/// Internal structure of a generator
struct Generator {
    value: u32,
    factor: u16,
}

impl Iterator for Generator {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        self.value = ((self.value as u64 * self.factor as u64) % 0x7FFFFFFF) as u32;
        Some(self.value as u16)
    }
}

/// Find judge's final count in dueling generators
pub fn star1(input: &str) -> String {
    let values = parse_values(input);
    count_judgement(
        Generator {
            factor: FACTOR_A,
            value: values[0],
        },
        Generator {
            factor: FACTOR_B,
            value: values[1],
        },
        STAR1_ITERATIONS,
    )
    .to_string()
}

/// Find judge's final count in dueling generators (with filtering)
pub fn star2(input: &str) -> String {
    let values = parse_values(input);
    count_judgement(
        Generator {
            factor: FACTOR_A,
            value: values[0],
        }
        .filter(|v| v % 4 == 0),
        Generator {
            factor: FACTOR_B,
            value: values[1],
        }
        .filter(|v| v % 8 == 0),
        STAR2_ITERATIONS,
    )
    .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "588");
    star_test!(me1, star1, ME, "577");

    star_test!(example1b, star2, IN1, "309");
    star_test!(me2, star2, ME, "316");

    const IN1: &str = indoc! {"
        Generator A starts with 65
        Generator B starts with 8921
    "};

    const ME: &str = indoc! {"
        Generator A starts with 618
        Generator B starts with 814
    "};
}
