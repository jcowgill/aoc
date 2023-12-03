use std::str::FromStr;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_ascii_whitespace().collect();
        match parts[0] {
            "addx" => Ok(Instruction::AddX(parts[1].parse().map_err(|_| ())?)),
            "noop" => Ok(Instruction::Noop),
            _ => Err(()),
        }
    }
}

pub fn star1(input: &str) -> String {
    let mut reg_x = vec![0, 1];

    for instr in input.lines().map(|l| l.parse::<Instruction>().unwrap()) {
        let x = *reg_x.last().unwrap();
        reg_x.push(x);

        if let Instruction::AddX(v) = instr {
            reg_x.push(x + v);
        }
    }

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|c| c as i32 * reg_x.get(c).unwrap_or_else(|| reg_x.last().unwrap()))
        .sum::<i32>()
        .to_string()
}

pub fn star2(input: &str) -> String {
    let mut crt = vec![false; 40 * 6];
    let mut cycle = 0;
    let mut do_cycle = |value| {
        let crt_pos = (cycle % 40) as i32;
        if (value - 1..=value + 1).contains(&crt_pos) {
            crt[cycle % (40 * 6)] = true;
        }
        cycle += 1;
    };

    let mut x = 1;
    do_cycle(x);
    for instr in input.lines().map(|l| l.parse::<Instruction>().unwrap()) {
        do_cycle(x);
        if let Instruction::AddX(v) = instr {
            x += v;
            do_cycle(x);
        }
    }

    crt.into_iter()
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.map(|v| if v { '#' } else { '.' }).collect::<String>())
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "-720");
    star_test!(example2a, star1, IN2, "13140");
    star_test!(me1, star1, ME1, "12520");

    star_test!(example1b, star2, IN1, OUT1);
    star_test!(example2b, star2, IN2, OUT2);
    star_test!(me2, star2, ME1, ME2);

    const IN1: &str = indoc! {"
        noop
        addx 3
        addx -5
    "};

    const IN2: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    const ME1: &str = indoc! {"
        addx 1
        noop
        addx 2
        noop
        addx 3
        addx 3
        addx 1
        addx 5
        addx 1
        noop
        noop
        addx 4
        noop
        noop
        addx -9
        addx 16
        addx -1
        noop
        addx 5
        addx -2
        addx 4
        addx -35
        addx 2
        addx 28
        noop
        addx -23
        addx 3
        addx -2
        addx 2
        addx 5
        addx -8
        addx 19
        addx -8
        addx 2
        addx 5
        addx 5
        addx -14
        addx 12
        addx 2
        addx 5
        addx 2
        addx -13
        addx -23
        noop
        addx 1
        addx 5
        addx -1
        addx 2
        addx 4
        addx -9
        addx 10
        noop
        addx 6
        addx -11
        addx 12
        addx 5
        addx -25
        addx 30
        addx -2
        addx 2
        addx -5
        addx 12
        addx -37
        noop
        noop
        noop
        addx 24
        addx -17
        noop
        addx 33
        addx -32
        addx 3
        addx 1
        noop
        addx 6
        addx -13
        addx 17
        noop
        noop
        noop
        addx 12
        addx -4
        addx -2
        addx 2
        addx 3
        addx 4
        addx -35
        addx -2
        noop
        addx 20
        addx -13
        addx -2
        addx 5
        addx 2
        addx 23
        addx -18
        addx -2
        addx 17
        addx -10
        addx 17
        noop
        addx -12
        addx 3
        addx -2
        addx 2
        noop
        addx 3
        addx 2
        noop
        addx -13
        addx -20
        noop
        addx 1
        addx 2
        addx 5
        addx 2
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx 1
        addx 2
        addx -18
        noop
        addx 26
        addx -1
        addx 6
        noop
        noop
        noop
        addx 4
        addx 1
        noop
        noop
        noop
        noop
    "};

    const ME2: &str = indoc! {"
        ####.#..#.###..####.###....##..##..#....
        #....#..#.#..#....#.#..#....#.#..#.#....
        ###..####.#..#...#..#..#....#.#....#....
        #....#..#.###...#...###.....#.#.##.#....
        #....#..#.#....#....#....#..#.#..#.#....
        ####.#..#.#....####.#.....##...###.####.
    "};

    const OUT1: &str = indoc! {"
        #####...................................
        ........................................
        ........................................
        ........................................
        ........................................
        ........................................
    "};

    const OUT2: &str = indoc! {"
        ##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######.....
    "};
}
