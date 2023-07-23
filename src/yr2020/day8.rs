#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Op {
    Add,
    Jump,
    Nop,
}

fn parse_input(input: &str) -> Vec<(Op, i32)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(' ').unwrap();
            (
                match a {
                    "acc" => Op::Add,
                    "jmp" => Op::Jump,
                    "nop" => Op::Nop,
                    _ => panic!("invalid opcode: {a}"),
                },
                b.parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn execute(program: Vec<(Op, i32)>) -> i32 {
    let mut seen = vec![false; program.len()];
    let mut pc = 0;
    let mut acc = 0;

    while seen.get(pc) == Some(&false) {
        seen[pc] = true;
        match program[pc] {
            (Op::Add, v) => {
                acc += v;
                pc += 1
            }
            (Op::Jump, v) => pc = (pc as i32 + v) as usize,
            (Op::Nop, _) => pc += 1,
        }
    }

    acc
}

pub fn star1(input: &str) -> String {
    execute(parse_input(input)).to_string()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Reachability {
    Unknown,
    Start,
    Dead,
    End,
}

fn scan_start(program: &[(Op, i32)]) -> Vec<Reachability> {
    let mut reach = vec![Reachability::Unknown; program.len()];
    let mut pc = 0;

    while reach[pc] == Reachability::Unknown {
        reach[pc] = Reachability::Start;
        if let (Op::Jump, v) = program[pc] {
            pc = (pc as i32 + v) as usize;
        } else {
            pc += 1;
        }
    }

    reach
}

fn scan_end(reach: &mut [Reachability], program: &[(Op, i32)], pos: usize) {
    let mut seen = Vec::new();
    let mut pc = pos;
    let mut status = Reachability::End;

    while pc < program.len() {
        if seen.contains(&pc) {
            status = Reachability::Dead;
            break;
        }

        match reach[pc] {
            Reachability::Unknown => {
                seen.push(pc);
                if let (Op::Jump, v) = program[pc] {
                    pc = (pc as i32 + v) as usize;
                } else {
                    pc += 1;
                }
            }
            Reachability::Start | Reachability::Dead => {
                status = Reachability::Dead;
                break;
            }
            Reachability::End => break,
        }
    }

    for i in seen {
        reach[i] = status;
    }
}

pub fn star2(input: &str) -> String {
    let mut program = parse_input(input);
    let mut reach = scan_start(&program);
    for i in 0..program.len() {
        scan_end(&mut reach, &program, i);
    }

    for i in 0..program.len() {
        if reach[i] == Reachability::Start {
            match program[i] {
                (Op::Jump, v) if reach[i + 1] == Reachability::End => {
                    program[i] = (Op::Nop, v);
                    return execute(program).to_string();
                }
                (Op::Nop, v) if reach[(i as i32 + v) as usize] == Reachability::End => {
                    program[i] = (Op::Jump, v);
                    return execute(program).to_string();
                }
                _ => (),
            }
        }
    }

    panic!("no solution");
}
