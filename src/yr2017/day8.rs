use std::cmp;
use std::collections::HashMap;
use std::i32;

/// A single instruction to be executed
struct Instruction<'a> {
    register: &'a str,
    value: i32,
    condition: fn(&i32, &i32) -> bool,
    condition_register: &'a str,
    condition_value: i32,
}

/// Type for the maps holding the list of registers
///  If a register is missing, it equals 0
type Registers<'a> = HashMap<&'a str, i32>;

/// Parses a single instruction
fn parse_instruction(line: &str) -> Instruction {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    // Perform some basic validation
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[3], "if");

    // Parse value
    let value = match tokens[1] {
        "inc" => tokens[2].parse::<i32>().unwrap(),
        "dec" => -tokens[2].parse::<i32>().unwrap(),
        _ => panic!("invalid inc/dec string found"),
    };

    // Parse condition
    let condition = match tokens[5] {
        "==" => i32::eq,
        "!=" => i32::ne,
        ">" => i32::gt,
        "<" => i32::lt,
        ">=" => i32::ge,
        "<=" => i32::le,
        _ => panic!("invalid comparison operator"),
    };

    Instruction {
        register: tokens[0],
        value,
        condition,
        condition_register: tokens[4],
        condition_value: tokens[6].parse().unwrap(),
    }
}

/// Updates the registers according to an instruction
fn execute_instruction<'a>(registers: &mut Registers<'a>, instruction: &Instruction<'a>) {
    // Evaluate condition, if true update register
    if (instruction.condition)(
        registers.get(instruction.condition_register).unwrap_or(&0),
        &instruction.condition_value,
    ) {
        *registers.entry(instruction.register).or_insert(0) += instruction.value;
    }
}

/// Execute a program, print register with largest value
pub fn star1(input: &str) -> String {
    let mut registers = Registers::new();

    for line in input.lines() {
        execute_instruction(&mut registers, &parse_instruction(line));
    }

    registers.values().max().unwrap().to_string()
}

/// Execute a program, print the largest value ever stored
pub fn star2(input: &str) -> String {
    let mut registers = Registers::new();
    let mut largest_value: i32 = i32::MIN;

    for line in input.lines() {
        // Execute instruction
        let instr = parse_instruction(line);
        execute_instruction(&mut registers, &instr);

        // Update largest value seen so far
        largest_value = cmp::max(
            largest_value,
            *registers.get(instr.register).unwrap_or(&i32::MIN),
        );
    }

    largest_value.to_string()
}
