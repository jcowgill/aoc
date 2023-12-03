use regex::Regex;
use std::collections::HashSet;

/// Applies one round of the MONAD mixing function
fn monad_mix(input: u32, z: u32, (a, b, c): (u32, i32, u32)) -> u32 {
    if ((z % 26) as i32 + b) as i64 == input.into() {
        z / a
    } else {
        (z / a) * 26 + input + c
    }
}

/// Find the maximum input which produces a valid serial number
fn find_max(
    seen: &mut HashSet<(u8, u32)>,
    params: &[(u32, i32, u32)],
    inputs: &(impl Iterator<Item = u8> + Clone),
    depth: u8,
    z: u32,
) -> Vec<u8> {
    // The mixer reduces Z by at most 26 times each iteration, so if
    // z is too large we can immediately discard this branch
    if z < 26_u32.saturating_pow(params.len() as u32 - u32::from(depth)) && seen.insert((depth, z))
    {
        for input in inputs.clone() {
            let value = monad_mix(input.into(), z, params[depth as usize]);
            if (depth as usize) < params.len() - 1 {
                let mut result = find_max(seen, params, inputs, depth + 1, value);
                if !result.is_empty() {
                    result.push(input);
                    return result;
                }
            } else if value == 0 {
                // Found a valid serial number!
                return vec![input];
            }
        }
    }

    Vec::new()
}

fn star_common(input: &str, inputs: &(impl Iterator<Item = u8> + Clone)) -> String {
    let regex_str = "inp w
mul x 0
add x z
mod x 26
div z ([0-9]+)
add x ([0-9-]+)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y ([0-9]+)
mul y x
add z y";

    let params: Vec<_> = Regex::new(regex_str)
        .unwrap()
        .captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            )
        })
        .collect();

    find_max(&mut HashSet::with_capacity(1 << 14), &params, inputs, 0, 0)
        .into_iter()
        .rev()
        .map(|b| char::from_digit(b.into(), 10).unwrap())
        .collect()
}

pub fn star1(input: &str) -> String {
    star_common(input, &(1..=9).rev())
}

pub fn star2(input: &str) -> String {
    star_common(input, &(1..=9))
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(me1, star1, ME, "98491959997994");

    star_test!(me2, star2, ME, "61191516111321");

    const ME: &str = indoc! {"
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 10
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 2
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 15
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 16
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 14
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 9
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 15
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 0
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -8
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 1
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 10
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 12
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -16
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 6
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -4
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 6
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 11
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 3
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -3
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 5
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 1
        add x 12
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 9
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -7
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 3
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -15
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 2
        mul y x
        add z y
        inp w
        mul x 0
        add x z
        mod x 26
        div z 26
        add x -7
        eql x w
        eql x 0
        mul y 0
        add y 25
        mul y x
        add y 1
        mul z y
        mul y 0
        add y w
        add y 3
        mul y x
        add z y
    "};
}
