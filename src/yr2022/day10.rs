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
