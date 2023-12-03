use crate::yr2017::processor::{program_step, ExecutionState, Instruction, StepResult};

/// Parses the input program into a list of instructions
fn parse_program(input: &str) -> Vec<Instruction> {
    let list: Vec<Instruction> = match input.lines().map(str::parse).collect() {
        Ok(list) => list,
        Err(_) => panic!("invalid program"),
    };

    // Validate program instructions
    for instr in list.iter() {
        match instr {
            &Instruction::Snd(_)
            | &Instruction::Rcv(_)
            | &Instruction::Set(_, _)
            | &Instruction::Add(_, _)
            | &Instruction::Mul(_, _)
            | &Instruction::Mod(_, _)
            | &Instruction::Jgz(_, _) => (),
            _ => panic!("invalid program"),
        }
    }

    list
}

/// Prints value of first completed rcv instruction
pub fn star1(input: &str) -> String {
    let program = parse_program(input);

    // Execute program until we block
    let mut state = ExecutionState::new(0);
    let mut last_sent = 0;
    loop {
        match program_step(&program, &mut state) {
            StepResult::Running => (),
            StepResult::Sent(val) => last_sent = val,
            StepResult::ReceiveBlocked => return last_sent.to_string(),
            StepResult::Terminated => panic!("program terminated ?!"),
        }
    }
}

/// Executes program twice simultaneously
pub fn star2(input: &str) -> String {
    let program = parse_program(input);
    let mut states = [ExecutionState::new(0), ExecutionState::new(1)];
    let mut stats_sent = [0, 0];
    let mut current = 0;

    // Execute programs, swapping if we become blocked
    loop {
        match program_step(&program, &mut states[current]) {
            StepResult::Running => (),
            StepResult::Sent(val) => {
                stats_sent[current] += 1;
                states[1 - current].blocked = false;
                states[1 - current].receive_queue.push_back(val);
            }
            StepResult::ReceiveBlocked => {
                // If both programs are blocked, we terminate, otherwise switch
                if states[0].blocked && states[1].blocked {
                    break;
                } else {
                    current = 1 - current;
                }
            }
            StepResult::Terminated => break,
        }
    }

    // Return number of times prog 1 sent a value
    stats_sent[1].to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "4");
    star_test!(me1, star1, ME, "7071");

    star_test!(example1b, star2, IN1, "1");
    star_test!(example2, star2, IN2, "3");
    star_test!(me2, star2, ME, "8001");

    const IN1: &str = indoc! {"
        set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2
    "};

    const IN2: &str = indoc! {"
        snd 1
        snd 2
        snd p
        rcv a
        rcv b
        rcv c
        rcv d
    "};

    const ME: &str = indoc! {"
        set i 31
        set a 1
        mul p 17
        jgz p p
        mul a 2
        add i -1
        jgz i -2
        add a -1
        set i 127
        set p 826
        mul p 8505
        mod p a
        mul p 129749
        add p 12345
        mod p a
        set b p
        mod b 10000
        snd b
        add i -1
        jgz i -9
        jgz a 3
        rcv b
        jgz b -1
        set f 0
        set i 126
        rcv a
        rcv b
        set p a
        mul p -1
        add p b
        jgz p 4
        snd a
        set a b
        jgz 1 3
        snd b
        set f 1
        add i -1
        jgz i -11
        snd a
        jgz f -16
        jgz a -19
    "};
}
