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

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    star_test!(example1a, star1, IN1, "5");
    star_test!(me1, star1, ME, "1087");

    star_test!(example1b, star2, IN1, "8");
    star_test!(me2, star2, ME, "780");

    const IN1: &str = indoc! {"
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
    "};

    const ME: &str = indoc! {"
        nop +456
        nop +38
        acc +9
        jmp +153
        acc +15
        nop +560
        jmp +452
        acc +26
        acc +42
        jmp +376
        acc -5
        acc +12
        acc -5
        jmp +15
        jmp +1
        acc -9
        jmp +533
        acc +19
        acc +33
        acc +34
        jmp -6
        nop +404
        nop +140
        acc +0
        jmp +123
        acc +45
        acc +0
        jmp +496
        jmp +487
        acc +9
        acc +34
        jmp +484
        acc +0
        acc -14
        jmp +466
        acc +40
        acc +6
        acc +30
        jmp +444
        nop +386
        jmp +215
        acc +43
        acc +5
        nop -4
        jmp +535
        jmp -13
        acc +3
        acc +7
        acc +49
        acc -1
        jmp +245
        acc +9
        acc +31
        nop +142
        jmp +554
        acc +3
        jmp +493
        nop +399
        jmp +232
        acc -16
        acc +33
        jmp +410
        acc +33
        acc +5
        acc -17
        jmp +272
        acc -3
        acc +37
        jmp +181
        jmp -12
        nop +344
        acc +5
        acc -16
        jmp +290
        nop +530
        acc +15
        acc +3
        jmp +343
        acc +2
        acc +19
        jmp +298
        acc +43
        acc +25
        acc -19
        jmp +439
        acc +43
        acc +45
        acc +20
        jmp +355
        acc +13
        acc +24
        acc -15
        nop +396
        jmp +215
        acc -7
        acc +17
        jmp +441
        acc -8
        acc -19
        jmp +505
        jmp +282
        acc -17
        acc -8
        acc +20
        jmp +359
        acc +26
        acc +14
        acc +47
        acc +3
        jmp +298
        acc +31
        nop +205
        acc +0
        acc +7
        jmp +389
        acc -5
        acc +47
        jmp +94
        acc -13
        jmp +358
        acc -13
        jmp +134
        acc +8
        acc -19
        jmp +312
        acc +43
        acc +17
        jmp +97
        jmp +48
        nop +253
        acc +48
        acc -7
        acc -2
        jmp +23
        acc +26
        acc +14
        acc -14
        acc +17
        jmp +18
        acc +14
        acc +8
        jmp +341
        acc +35
        jmp +227
        acc +15
        acc -7
        jmp -95
        acc -19
        jmp -59
        jmp -31
        acc -6
        acc -4
        acc +24
        jmp +84
        acc -15
        jmp +82
        nop +74
        acc +8
        acc +9
        acc +13
        jmp +194
        jmp +376
        acc +34
        nop -16
        jmp -90
        acc +4
        acc +43
        nop +215
        jmp -147
        acc +0
        acc +11
        acc -15
        acc +23
        jmp +130
        acc +40
        jmp +106
        acc -4
        acc -18
        acc +18
        nop +329
        jmp +230
        acc +19
        nop +172
        acc +43
        jmp +304
        acc +44
        nop +213
        nop +195
        acc +6
        jmp -79
        acc +41
        acc -11
        acc +18
        acc -9
        jmp -25
        acc +27
        acc -6
        acc +31
        jmp -56
        acc +5
        acc +12
        acc +32
        acc +34
        jmp -189
        acc +32
        acc +5
        acc -16
        jmp +301
        nop +108
        nop -108
        jmp -141
        acc -12
        jmp +273
        acc +3
        jmp +140
        acc +7
        acc -11
        acc -17
        nop +194
        jmp -122
        acc -14
        nop +186
        acc +24
        jmp +277
        nop +341
        acc +18
        jmp -64
        acc +45
        acc +42
        jmp +52
        acc +39
        nop +91
        nop -8
        jmp +217
        acc +44
        acc +15
        jmp +72
        acc +24
        jmp -231
        acc -16
        nop +55
        nop +262
        acc +40
        jmp +234
        jmp -14
        acc +31
        nop -177
        acc +40
        jmp +343
        acc -8
        jmp -169
        acc +30
        acc +12
        acc -11
        jmp +41
        acc +9
        acc -9
        jmp +65
        acc +38
        acc +14
        jmp +335
        acc -19
        acc +38
        acc +16
        acc -11
        jmp +230
        jmp -71
        acc +48
        acc -13
        nop -255
        jmp +1
        jmp -220
        acc +2
        jmp +157
        jmp -105
        acc -16
        acc -5
        jmp -196
        acc +30
        jmp +139
        jmp +83
        acc -3
        acc -12
        jmp +254
        jmp -60
        acc +33
        jmp -37
        acc +17
        acc -14
        jmp +93
        nop +178
        acc +38
        acc +47
        jmp -89
        jmp +271
        acc +43
        acc +32
        jmp -240
        acc +26
        acc +32
        acc +30
        nop +284
        jmp +169
        acc -7
        acc +37
        jmp +102
        acc +4
        jmp +86
        jmp -123
        acc +0
        acc -14
        acc +18
        jmp +1
        jmp -5
        jmp -36
        jmp +148
        acc -17
        acc -14
        acc +28
        acc +15
        jmp +79
        jmp -289
        acc +42
        acc -5
        acc +13
        jmp +240
        acc -10
        acc -18
        acc -16
        jmp +103
        acc +21
        jmp +32
        nop +118
        acc +22
        acc -16
        acc +15
        jmp -186
        acc -2
        acc -14
        acc +22
        acc +16
        jmp +73
        acc -6
        jmp -225
        acc -18
        nop +113
        acc +50
        acc -6
        jmp +181
        acc +41
        jmp +1
        nop +92
        acc +23
        jmp +190
        acc +39
        acc +0
        acc +33
        jmp +111
        nop -63
        nop -81
        acc +9
        acc +35
        jmp +50
        acc +11
        jmp -295
        nop +230
        acc +34
        acc +12
        acc +47
        jmp +126
        acc +0
        nop -1
        acc +19
        acc -16
        jmp -360
        acc +29
        acc -2
        jmp -110
        acc +2
        acc +50
        jmp -36
        jmp -107
        jmp +178
        acc -11
        jmp +181
        nop +115
        nop +186
        jmp +95
        jmp +1
        nop +148
        acc +2
        acc +49
        jmp +173
        acc +38
        jmp +178
        acc +28
        acc +6
        acc +15
        jmp +110
        acc +49
        nop +100
        jmp +57
        acc +45
        nop +65
        acc +43
        acc +12
        jmp -272
        jmp -260
        nop +100
        jmp -224
        jmp +142
        jmp +52
        jmp -34
        jmp -110
        acc +35
        nop -112
        jmp +16
        jmp -18
        jmp -157
        jmp +81
        acc +1
        jmp -107
        acc +16
        acc +23
        jmp -255
        acc +22
        jmp +42
        nop +168
        acc +41
        jmp -311
        jmp -163
        jmp +118
        nop +4
        acc +18
        jmp +54
        jmp -414
        nop -181
        acc +10
        acc +23
        jmp -321
        nop -322
        acc -9
        jmp +101
        nop -7
        acc +35
        acc +46
        jmp -312
        nop +64
        nop -386
        jmp -280
        acc +16
        jmp -156
        acc +13
        nop -131
        jmp +1
        jmp -416
        jmp +15
        jmp -94
        jmp -330
        nop +93
        nop -205
        acc +48
        jmp -19
        jmp -70
        nop +21
        acc -5
        acc +19
        jmp +62
        acc +22
        jmp -448
        jmp -77
        acc +26
        acc -2
        jmp +70
        acc -2
        acc +21
        jmp -195
        nop -114
        jmp +107
        acc +37
        acc +6
        jmp -436
        acc +48
        jmp +96
        jmp -121
        acc +0
        jmp -74
        jmp +1
        acc +27
        acc +2
        jmp -279
        acc +7
        acc +0
        jmp +1
        jmp -413
        acc +6
        jmp -180
        acc +18
        acc +10
        jmp -437
        jmp -338
        nop -456
        jmp -463
        acc +1
        nop -54
        jmp -168
        acc +27
        jmp -479
        acc +42
        jmp -408
        jmp +85
        acc -16
        acc +24
        jmp -391
        jmp -206
        nop +8
        jmp +1
        acc +38
        nop -473
        jmp -94
        acc +10
        acc -14
        jmp -425
        acc +17
        nop -208
        acc +39
        jmp -265
        acc +3
        jmp -284
        acc +19
        acc +5
        nop -111
        acc +22
        jmp -309
        acc +12
        acc +39
        jmp -151
        acc +33
        acc -14
        jmp -450
        acc +16
        nop +50
        jmp -188
        acc -13
        acc +15
        acc +4
        jmp -484
        acc +27
        jmp -98
        acc +34
        jmp -120
        jmp -537
        acc +43
        acc -8
        acc -6
        jmp -405
        acc -8
        nop -179
        acc -11
        jmp -264
        acc +24
        jmp -280
        acc -6
        acc +1
        jmp -353
        acc -18
        jmp -58
        acc +1
        acc -7
        acc -2
        acc +44
        jmp -115
        nop -328
        acc +27
        acc +2
        jmp +20
        acc +14
        acc +34
        jmp -460
        nop -445
        acc -9
        acc +24
        acc -11
        jmp -72
        jmp -434
        jmp -370
        acc +35
        acc +43
        acc +45
        acc +44
        jmp -287
        jmp -546
        nop -474
        acc -6
        jmp -357
        nop -163
        nop -218
        nop -342
        jmp -570
        acc +44
        acc +4
        acc +35
        acc +6
        jmp -541
        jmp -274
        acc +48
        acc -18
        jmp -171
        acc -13
        acc -14
        acc +25
        acc +26
        jmp +1
    "};
}
