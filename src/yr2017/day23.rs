use crate::yr2017::processor::{program_step, ExecutionState, Instruction, RegImm, StepResult};

/// Parses the input program into a list of instructions
fn parse_program(input: &str) -> Vec<Instruction> {
    let list: Vec<Instruction> = match input.lines().map(str::parse).collect() {
        Ok(list) => list,
        Err(_) => panic!("invalid program"),
    };

    // Validate program instructions
    for instr in list.iter() {
        match instr {
            &Instruction::Set(_, _)
            | &Instruction::Sub(_, _)
            | &Instruction::Mul(_, _)
            | &Instruction::Jnz(_, _) => (),
            _ => panic!("invalid program"),
        }
    }

    list
}

/// Counts number of executed mul instructions
pub fn star1(input: &str) -> String {
    let program = parse_program(input);

    // Execute program until termination
    let mut state = ExecutionState::new(0);
    let mut mul_instructions = 0;
    loop {
        // Count mul instructions we're about to execute
        if let Some(&Instruction::Mul(_, _)) = program.get(state.pc as usize) {
            mul_instructions += 1
        }

        // Step one instruction
        match program_step(&program, &mut state) {
            StepResult::Running => (),
            StepResult::Terminated => break,
            _ => panic!("snd/rcv executed ?!"),
        }
    }

    mul_instructions.to_string()
}

/// Perform special optimization for star2
fn star2_optimize(program: &mut [Instruction]) {
    // We optimize the specific inner loop used in the star2 program
    //  The inner loop effectively emulates a mod instruction very slowly

    while let Some((i, b, d, e, f, g)) = program
        .windows(10)
        .enumerate()
        .filter_map(|(i, window)| {
            // Slice pattern matching not yet stablized so we have to use an if-let chain
            if let Instruction::Set(e, RegImm::Imm(2)) = window[0] {
                if let Instruction::Set(g, RegImm::Reg(d)) = window[1] {
                    if let Instruction::Mul(g1, RegImm::Reg(e1)) = window[2] {
                        if let Instruction::Sub(g2, RegImm::Reg(b)) = window[3] {
                            if let Instruction::Jnz(RegImm::Reg(g3), RegImm::Imm(2)) = window[4] {
                                if let Instruction::Set(f, RegImm::Imm(0)) = window[5] {
                                    if let Instruction::Sub(e2, RegImm::Imm(-1)) = window[6] {
                                        if let Instruction::Set(g4, RegImm::Reg(e3)) = window[7] {
                                            if let Instruction::Sub(g5, RegImm::Reg(b1)) = window[8]
                                            {
                                                if let Instruction::Jnz(
                                                    RegImm::Reg(g6),
                                                    RegImm::Imm(-8),
                                                ) = window[9]
                                                {
                                                    if b == b1
                                                        && e == e1
                                                        && e == e2
                                                        && e == e3
                                                        && g == g1
                                                        && g == g2
                                                        && g == g3
                                                        && g == g4
                                                        && g == g5
                                                        && g == g6
                                                    {
                                                        return Some((i, b, d, e, f, g));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            None
        })
        .next()
    {
        program[i] = Instruction::Set(g, RegImm::Reg(b));
        program[i + 1] = Instruction::Mod(g, RegImm::Reg(d));
        program[i + 2] = Instruction::Jnz(RegImm::Reg(g), RegImm::Imm(2));
        program[i + 3] = Instruction::Set(f, RegImm::Imm(0));
        program[i + 4] = Instruction::Set(e, RegImm::Reg(b));
        program[i + 5] = Instruction::Set(g, RegImm::Imm(0));

        // Insert some nops until we've used up 10 instructions
        program[i + 6] = Instruction::Set(g, RegImm::Imm(0));
        program[i + 7] = Instruction::Set(g, RegImm::Imm(0));
        program[i + 8] = Instruction::Set(g, RegImm::Imm(0));
        program[i + 9] = Instruction::Set(g, RegImm::Imm(0));
    }
}

/// Final value in h register
pub fn star2(input: &str) -> String {
    let mut program = parse_program(input);

    // Optimize inner loop
    star2_optimize(&mut program);

    // Execute program until termination and read h register
    let mut state = ExecutionState::new(0);
    state.regs[0] = 1;
    loop {
        // Step one instruction
        match program_step(&program, &mut state) {
            StepResult::Running => (),
            StepResult::Terminated => break,
            _ => panic!("snd/rcv executed ?!"),
        }
    }

    state.regs[(b'h' - b'a') as usize].to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(me1, star1, ME, "8281");

    star_test!(me2, star2, ME, "911");

    const ME: &str = indoc! {"
        set b 93
        set c b
        jnz a 2
        jnz 1 5
        mul b 100
        sub b -100000
        set c b
        sub c -17000
        set f 1
        set d 2
        set e 2
        set g d
        mul g e
        sub g b
        jnz g 2
        set f 0
        sub e -1
        set g e
        sub g b
        jnz g -8
        sub d -1
        set g d
        sub g b
        jnz g -13
        jnz f 2
        sub h -1
        set g b
        sub g c
        jnz g 2
        jnz 1 3
        sub b -17
        jnz 1 -23
    "};
}
