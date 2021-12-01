///! Generalisation of the processor used in days 18 and 23
use std::collections::VecDeque;
use std::str::FromStr;

/// The current state while running a program
pub struct ExecutionState {
    /// Program counter
    pub pc: i64,

    /// Register contents
    pub regs: [i64; 26],

    /// Receive queue for this program (values send to me end up here)
    pub receive_queue: VecDeque<i64>,

    /// If true, program is blocked waiting for input
    pub blocked: bool,
}

impl ExecutionState {
    /// Returns a new execution state
    pub fn new(pid: i64) -> ExecutionState {
        let mut state = ExecutionState {
            pc: 0,
            regs: [0; 26],
            receive_queue: VecDeque::new(),
            blocked: false,
        };
        state.regs[(b'p' - b'a') as usize] = pid;
        state
    }
}

/// Trait implemented by objects which have a "value" within an execution context
trait ValueQueriable {
    /// Returns the value of this object
    fn get_value(&self, _: &ExecutionState) -> i64;
}

/// A machine register name
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Register(u8);

impl FromStr for Register {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        if let Some(c) = s.chars().next() {
            if s.len() == 1 && c >= 'a' && c <= 'z' {
                return Ok(Register(c as u8 - b'a'));
            }
        }

        Err(())
    }
}

impl ValueQueriable for Register {
    fn get_value(&self, state: &ExecutionState) -> i64 {
        state.regs[self.0 as usize]
    }
}

/// Either a register name or immediate value
#[derive(Clone, Copy)]
pub enum RegImm {
    Reg(Register),
    Imm(i64),
}

impl FromStr for RegImm {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        match s.parse::<i64>() {
            Ok(value) => Ok(RegImm::Imm(value)),
            Err(_) => Ok(RegImm::Reg(s.parse::<Register>()?)),
        }
    }
}

impl ValueQueriable for RegImm {
    fn get_value(&self, state: &ExecutionState) -> i64 {
        match *self {
            RegImm::Reg(reg) => reg.get_value(state),
            RegImm::Imm(val) => val,
        }
    }
}

pub enum Instruction {
    Snd(RegImm),
    Rcv(Register),
    Set(Register, RegImm),
    Add(Register, RegImm),
    Sub(Register, RegImm),
    Mul(Register, RegImm),
    Mod(Register, RegImm),
    Jgz(RegImm, RegImm),
    Jnz(RegImm, RegImm),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        // Validate length first
        if parts.len() < 2 {
            return Err(());
        }
        if parts[0] == "snd" || parts[0] == "rcv" {
            if parts.len() != 2 {
                return Err(());
            }
        } else if parts.len() != 3 {
            return Err(());
        }

        // Parse the input parts
        match parts[0] {
            "snd" => Ok(Instruction::Snd(parts[1].parse()?)),
            "rcv" => Ok(Instruction::Rcv(parts[1].parse()?)),
            "set" => Ok(Instruction::Set(parts[1].parse()?, parts[2].parse()?)),
            "add" => Ok(Instruction::Add(parts[1].parse()?, parts[2].parse()?)),
            "sub" => Ok(Instruction::Sub(parts[1].parse()?, parts[2].parse()?)),
            "mul" => Ok(Instruction::Mul(parts[1].parse()?, parts[2].parse()?)),
            "mod" => Ok(Instruction::Mod(parts[1].parse()?, parts[2].parse()?)),
            "jgz" => Ok(Instruction::Jgz(parts[1].parse()?, parts[2].parse()?)),
            "jnz" => Ok(Instruction::Jnz(parts[1].parse()?, parts[2].parse()?)),
            _ => Err(()),
        }
    }
}

/// The result of executing a program step
pub enum StepResult {
    /// Program still running
    Running,

    /// Program has sent a value
    Sent(i64),

    /// Program executed a receive instruction with an empty queue
    ReceiveBlocked,

    /// Program has terminated
    Terminated,
}

/// Executes the next instruction in a program
pub fn program_step(program: &[Instruction], state: &mut ExecutionState) -> StepResult {
    // Check if pc is in range
    if state.pc < 0 || state.pc as usize >= program.len() {
        return StepResult::Terminated;
    }

    // Advance pc (makes below code simpler)
    state.pc += 1;

    // Execute current instruction
    match program[(state.pc - 1) as usize] {
        Instruction::Snd(val) => StepResult::Sent(val.get_value(state)),
        Instruction::Rcv(reg) => match state.receive_queue.pop_front() {
            Some(value) => {
                state.blocked = false;
                state.regs[reg.0 as usize] = value;
                StepResult::Running
            }
            None => {
                state.blocked = true;
                state.pc -= 1;
                StepResult::ReceiveBlocked
            }
        },
        Instruction::Set(reg, val) => {
            state.regs[reg.0 as usize] = val.get_value(state);
            StepResult::Running
        }
        Instruction::Add(reg, val) => {
            state.regs[reg.0 as usize] += val.get_value(state);
            StepResult::Running
        }
        Instruction::Sub(reg, val) => {
            state.regs[reg.0 as usize] -= val.get_value(state);
            StepResult::Running
        }
        Instruction::Mul(reg, val) => {
            state.regs[reg.0 as usize] *= val.get_value(state);
            StepResult::Running
        }
        Instruction::Mod(reg, val) => {
            state.regs[reg.0 as usize] %= val.get_value(state);
            StepResult::Running
        }
        Instruction::Jgz(cond, offset) => {
            if cond.get_value(state) > 0 {
                state.pc += offset.get_value(state) - 1;
            };
            StepResult::Running
        }
        Instruction::Jnz(cond, offset) => {
            if cond.get_value(state) != 0 {
                state.pc += offset.get_value(state) - 1;
            };
            StepResult::Running
        }
    }
}
