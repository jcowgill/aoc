use yr2017::processor::{ExecutionState, Instruction, StepResult, program_step};

/// Parses the input program into a list of instructions
fn parse_program(input: &str) -> Vec<Instruction> {
    let list: Vec<Instruction> = match input.lines().map(str::parse).collect() {
        Ok(list) => list,
        Err(_)   => panic!("invalid program")
    };

    // Validate program instructions
    for instr in list.iter() {
        match instr {
            &Instruction::Snd(_) | &Instruction::Rcv(_) |
            &Instruction::Set(_, _) | &Instruction::Add(_, _) |
            &Instruction::Mul(_, _) | &Instruction::Mod(_, _) |
            &Instruction::Jgz(_, _) => (),
            _ => panic!("invalid program"),
        }
    };

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
            StepResult::Running        => (),
            StepResult::Sent(val)      => last_sent = val,
            StepResult::ReceiveBlocked => return last_sent.to_string(),
            StepResult::Terminated     => panic!("program terminated ?!"),
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
            },
            StepResult::ReceiveBlocked => {
                // If both programs are blocked, we terminate, otherwise switch
                if states[0].blocked && states[1].blocked {
                    break;
                } else {
                    current = 1 - current;
                }
            },
            StepResult::Terminated => break
        }
    }

    // Return number of times prog 1 sent a value
    stats_sent[1].to_string()
}
