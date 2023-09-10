use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Instruction {
    Mask(u64, u64),
    Set(u16, u64),
}

fn parse_instr(s: &str) -> Instruction {
    let (a, b) = s.split_once('=').unwrap();
    if a.trim() == "mask" {
        let mut set = 0;
        let mut reset = 0;
        for c in b.trim().chars() {
            set <<= 1;
            reset <<= 1;
            match c {
                '1' => set |= 1,
                '0' => reset |= 1,
                'X' => (),
                _ => panic!("invalid mask character '{c}'"),
            }
        }
        Instruction::Mask(set, reset)
    } else if let Some(addr_str) = a.strip_prefix("mem[") {
        let addr = addr_str.trim_end_matches([' ', ']']).parse().unwrap();
        let value = b.trim().parse().unwrap();
        Instruction::Set(addr, value)
    } else {
        panic!("invalid instruction {}", a.trim());
    }
}

pub fn star1(input: &str) -> String {
    let mut memory = HashMap::new();
    let mut mask_set = 0;
    let mut mask_reset = 0;

    for line in input.lines() {
        match parse_instr(line) {
            Instruction::Mask(s, r) => {
                mask_set = s;
                mask_reset = r;
            }
            Instruction::Set(a, v) => {
                memory.insert(a, (v | mask_set) & !mask_reset);
            }
        }
    }

    memory.into_values().sum::<u64>().to_string()
}

pub fn star2(input: &str) -> String {
    let mut memory = HashMap::new();
    let mut mask_set = 0;
    let mut floating = 0;
    let mut floating_bits = Vec::new();

    for line in input.lines() {
        match parse_instr(line) {
            Instruction::Mask(s, r) => {
                floating = ((1 << 36) - 1) & !s & !r;
                floating_bits = (0..36)
                    .map(|i| 1 << i)
                    .filter(|m| floating & m != 0)
                    .collect();
                mask_set = s;
            }
            Instruction::Set(a, v) => {
                for i in 0..(1 << floating_bits.len()) {
                    let floating_set = floating_bits
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| i & (1 << j) != 0)
                        .map(|(_, v)| v)
                        .sum::<u64>();
                    let floating_reset = floating ^ floating_set;
                    let addr = (u64::from(a) | mask_set | floating_set) & !floating_reset;
                    memory.insert(addr, v);
                }
            }
        }
    }

    memory.into_values().sum::<u64>().to_string()
}
